use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse, client::Client};
use actix_service::{Service, Transform};

use anyhow::Result;
use futures::future::{ok, Future, Ready};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use actix_web::http::Uri;
use std::str::FromStr;
use std::convert::TryFrom;

use serde::{Deserialize};
use openssl::sign::{Verifier};
use openssl::rsa::Padding;
use openssl::pkey::{PKey};
use openssl::hash::MessageDigest;
use openssl::base64::{decode_block};

#[derive(Debug)]
struct SignatureInput {
    alg: String,
    created: u64,
    expires: u64,
    key_id: String,
    covered_content: String,
}

// This is ALL boilerplate for a middleware,
// TODO: Move to another file when its done
// https://github.com/casbin-rs/actix-casbin-auth/blob/master/src/middleware.rs
// a link to a helpful implementation of a middleware thats kinda auth

pub struct Keys {
    pub private: Vec<u8>,
    pub public: Vec<u8>,
}

pub struct RequestAuth;

impl<S: 'static, B> Transform<S> for RequestAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestAuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct RequestAuthMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for RequestAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut srv = self.service.clone();

        Box::pin(async move {
            // just check if it wants our key, if so let it on through!
            if req.path() == "/api/key" {
                return srv.call(req).await
            }

            let headers = req.headers();
            use actix_web::http::{HeaderName, HeaderValue};

            if let Some(token_field) = headers.get("Authorization") {
                if let Ok(token_str) = token_field.to_str() {
                    if token_str.len() > 8 {
                        if let Ok(user_id) = crate::auth::decode_jwt(&token_str[7..]) {
                            req.headers_mut().insert(HeaderName::from_static("user-id"), HeaderValue::from_str(&user_id).unwrap());
                            return srv.call(req).await
                        }
                    }
                }
                return srv.call(req).await
            } else if headers.contains_key("Signature") && headers.contains_key("Signature-Input") {

                let mut sig_input_struct = SignatureInput {
                    alg: String::from(""),
                    created: 0,
                    expires: 0,
                    key_id: String::from(""),
                    covered_content: String::from(""),
                };

                // get header value for sig input
                let signature_input = headers.get("Signature-Input").unwrap().to_str().unwrap();

                // TODO: check for 0 splits, could be caught later potentially. - Darren
                let iter_signature_input = signature_input.split(";");

                // build struct and do soft validation on signature-input header contents
                // TODO: can make this better by using an enum but im lazy x - Darren
                // TODO: realistically only the created and expires will fail to parse, but these
                // aren't in the protocl so we can kinda of ignore them for now - Darren
                for entry in iter_signature_input {
                    let trim_entry = entry.trim();
                    if trim_entry.starts_with("sig1=") {
                        sig_input_struct.covered_content = trim_entry.strip_prefix("sig1=").unwrap().to_string();
                    }
                    else if trim_entry.starts_with("alg=") {
                        sig_input_struct.alg = trim_entry.strip_prefix("alg=").unwrap().to_string();
                    }
                    else if trim_entry.starts_with("created=") {
                        sig_input_struct.created = trim_entry.strip_prefix("created=").unwrap().parse::<u64>().unwrap();
                    }
                    else if trim_entry.starts_with("expires=") {
                        sig_input_struct.expires = trim_entry.strip_prefix("expires=").unwrap().parse::<u64>().unwrap();
                    }
                    else if trim_entry.starts_with("keyId=") {
                        sig_input_struct.key_id = trim_entry.strip_prefix("keyId=").unwrap().to_string();
                    }
                    else {
                        // invalid attribute used in request
                        let body = format!("Invalid signature-input attribute: {}", trim_entry);
                        return Ok(req.into_response(HttpResponse::BadRequest().body(body).into_body()))
                    }
                }
                // check covered_content / sig1= is valid with headers

                // check starts and ends with parenthesis
                if !sig_input_struct.covered_content.starts_with("(") || !sig_input_struct.covered_content.ends_with(")") {
                    // if not, then error
                    let body = format!("Invalid sig1= not surrounded by parenthesis: {}", sig_input_struct.covered_content);
                    return Ok(req.into_response(HttpResponse::BadRequest().body(body).into_body()))
                }
                sig_input_struct.covered_content = sig_input_struct.covered_content.strip_prefix("(").unwrap().to_string();
                sig_input_struct.covered_content = sig_input_struct.covered_content.strip_suffix(")").unwrap().to_string();
                let iter = sig_input_struct.covered_content.split(",");

                // check each one against headers, dealing with speical * cases
                // meanwhile building signature input

                /* * cases
                 * *request-target
                 * *created - not impl
                 * *expires - not impl
                 */

                let mut signature_strings = Vec::new();
                for field in iter {
                    let field_trim = field.trim();

                    let req_tar_string = "*request-target";
                    if field_trim.starts_with(req_tar_string) {
                        signature_strings.push(format!("{}: {}", req_tar_string, req.path()));
                    }
                    // TODO: if statements for *created and *expires

                    else {
                        // check field against headers, if exist add, else error
                        if !headers.contains_key(field_trim) {
                            let body = format!("No header exists for: {}", field_trim);
                            return Ok(req.into_response(HttpResponse::BadRequest().body(body).into_body()))
                        }

                        // add to vector
                        signature_strings.push(
                            // TODO: header might not contain valid ascii, worker will panic right
                            // now, should return error if fail
                            format!("{}: {}", field_trim, headers.get(field_trim).unwrap().to_str().unwrap())
                        );
                    }
                }

                // used so serde can deal with the key: <key> from the api response
                #[derive(Deserialize)]
                struct Key {
                    key: String,
                };

                // TODO: potentially cache key in db per domain
                // TODO: extract domain from keyId

                // makes request for key, returning an error if this fails, or key isn't a valid
                // string
               let client = Client::default();
               let unparsed_key = match client.get(&sig_input_struct.key_id).send().await {
                   Ok(mut response) => match response.json::<Key>().await {
                       Ok(key) => key,
                       Err(_) => return Ok(req.into_response(HttpResponse::BadRequest().body("Error parsing response body").into_body())),
                   },
                   Err(req_err) => return Ok(req.into_response(HttpResponse::BadRequest().body(format!("Error while making key req to keyId: {}", req_err)).into_body()))
               };

               // build string to sign
               let size = signature_strings.len();
               let mut index: usize = 1;
               let mut string_to_sign = String::with_capacity(300); // arbritary alloc, should be enough for most signature inputs without any reallocs
               for field in signature_strings {
                   if size == index {
                       string_to_sign.push_str(&field);
                   }
                   else {
                       string_to_sign.push_str(&format!("{}\n", field));
                       index = index + 1;
                   }
               }

            // have checked signature exists, value should be a valid string (hopefully)
            let enc_signature = req.headers().get("signature").unwrap().to_str().unwrap();

            let public_key_parsed = match PKey::public_key_from_pem(&unparsed_key.key.as_ref()) {
                Ok(key) => key,
                Err(_) =>return Ok(req.into_response(HttpResponse::BadRequest().body("Error parsing public key, invalid format(?)").into_body())),
            };

            let mut verifier = Verifier::new(MessageDigest::sha512(), &public_key_parsed).unwrap();
            verifier.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
            verifier.update(string_to_sign.as_ref()).unwrap();

            let denc_signature = match decode_block(&enc_signature) {
                Ok(decoded) => decoded,
                Err(e) =>return Ok(req.into_response(HttpResponse::BadRequest().body(format!("Error decoding signature from base64: {}", e)).into_body())),
            };

            if verifier.verify(&denc_signature).unwrap() {
                return srv.call(req).await
            } else {
                return Ok(req.into_response(HttpResponse::BadRequest().body("Error signing signature: may not match").into_body()))
            }

            }
            else {
                return Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
            }
        })
    }
}
