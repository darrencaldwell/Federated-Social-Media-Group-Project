use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse, client::Client, web::Data};
use actix_service::{Service, Transform};

use anyhow::Result;
use futures::future::{ok, Future, Ready};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use base64;
use ring::{
    rand,
    signature::{self, KeyPair},
};

use serde::{Serialize, Deserialize};
use openssl::sign::{Signer, Verifier};
use openssl::rsa::Padding;
use openssl::pkey::{PKey, Private};
use openssl::hash::MessageDigest;
use openssl::base64::{encode_block, decode_block};

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
            let headers = req.headers();
            println!("Hi from request! {:?}", headers);
            //println!("{}, {}", &req.path(), &req.method());
            use actix_web::http::{HeaderName, HeaderValue};

            if let Some(token_field) = headers.get("Authorization") {
                if let Ok(token_str) = token_field.to_str() {
                    if token_str.len() > 8 {
                        if let Ok(user_id) = crate::auth::decode_jwt(&token_str[7..]) {
                            req.headers_mut().insert(HeaderName::from_static("user_id"), HeaderValue::from_str(&user_id).unwrap());
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
                // TODO: handle errors better, as of now unwraps simply panic and kill the worker,
                // telling nothing to the sender. - Darren
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
                        // TODO: parse this in the struct as an http::Uri for better error checking
                        sig_input_struct.key_id = trim_entry.strip_prefix("keyId=").unwrap().to_string();
                    }
                    else {
                        // invalid attribute used in request
                        let body = format!("Invalid signature-input attribute: {}", trim_entry);
                        println!("Error: request for: {}, {}", req.path(), body);
                        return Ok(req.into_response(HttpResponse::BadRequest().body(body).into_body()))
                    }
                }
                // check covered_content / sig1= is valid with headers

                // check starts and ends with parenthesis
                if !sig_input_struct.covered_content.starts_with("(") || !sig_input_struct.covered_content.ends_with(")") {
                    // error
                    let body = format!("Invalid sig1= not surrounded by parenthesis: {}", sig_input_struct.covered_content);
                    println!("Error: request for: {}, {}", req.path(), body);
                    return Ok(req.into_response(HttpResponse::BadRequest().body(body).into_body()))
                }
                // remove parenthesis
                sig_input_struct.covered_content = sig_input_struct.covered_content.strip_prefix("(").unwrap().to_string();
                sig_input_struct.covered_content = sig_input_struct.covered_content.strip_suffix(")").unwrap().to_string();
                // seperate by commas
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
                            println!("Error: request for: {}, {}", req.path(), body);
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

                #[derive(Deserialize)]
                struct Key {
                    key: String,
                };

                // make request for key
                // TODO: store key in database, query that, then make request, or be a bad noodle
                // and just always ask for it
                //
                // honestly this client code is horrendous and it's not even my fault
               let client = Client::default();
               let response = client.get(sig_input_struct.key_id)
                  .send()     // <- Send request
                  .await      // <- Wait for response
                  .unwrap()
                  .json::<Key>()
                  .await;

               // TODO: check we got a valid key response
               //let key_pem = response.unwrap().key;

               // build string to sign
               let size = signature_strings.len();
               let mut index: usize = 1;
               let mut string_to_sign = String::with_capacity(300); // arbritary alloc, should be enough for most signature inputs without any reallocs
               for field in signature_strings {
                   if size == index {
                       // just add
                       string_to_sign.push_str(&field);
                   }
                   else {
                       string_to_sign.push_str(&format!("{}\n", field));
                       index = index + 1;
                   }
               }

               println!("request: {}", string_to_sign);

            let key_pair = req.app_data::<Data<PKey<Private>>>().unwrap().clone();
            let public_key_pem = key_pair.public_key_to_pem().unwrap();
            println!("{:?}", std::str::from_utf8(&public_key_pem));

            let mut signer = Signer::new(MessageDigest::sha512(), &key_pair).unwrap();
            signer.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
            signer.update(string_to_sign.as_ref()).unwrap();
            let signature = signer.sign_to_vec().unwrap();
            let enc_signature = encode_block(&signature);

            let public_key_read_in = PKey::public_key_from_pem(&public_key_pem).unwrap();
            let mut verifier = Verifier::new(MessageDigest::sha512(), &public_key_read_in).unwrap();
            verifier.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
            verifier.update(string_to_sign.as_ref()).unwrap();

            let denc_signature = decode_block(&enc_signature).unwrap();
            if verifier.verify(&denc_signature).unwrap() {
                println!("this worked!");
                return srv.call(req).await
            } else {
                return Ok(req.into_response(HttpResponse::BadRequest().body("Error signing signature, may not match").into_body()))
            }

            }
            else {
                // creates an error response and sends it back to the sender
                return Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
            }
        })
    }
}
