use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse, client::Client};
use actix_service::{Service, Transform};

use anyhow::Result;
use futures::future::{ok, Future, Ready};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use regex::Regex;

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

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut srv = self.service.clone();

        Box::pin(async move {
            let headers = req.headers();
            println!("Hi from request! {:?}", headers);
            println!("{}, {}", req.path(), req.method());

            if headers.contains_key("Authorization") {
                let token = headers.get("Authorization").unwrap();
                    // TODO: VERIFY TOKEN
                    srv.call(req).await // basically, carry out the request, route it to our functions? etc maybe idk
            }
            else if headers.contains_key("Signature") && headers.contains_key("Signature-Input") {

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
                // check covered_content is valid with headers

                // make request for key
               let client = Client::default();
               // Create request builder and send request
               let response = client.get(sig_input_struct.key_id)
                  .send()     // <- Send request
                  .await;     // <- Wait for response

               println!("Response: {:?}", response);

               // if got key, build string to check signature against
               // authorise or don't

                srv.call(req).await // basically, carry out the request, route it to our functions? etc maybe idk
            }
            else {
                // creates an error response and sends it back to the sender
                return Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
            }
        })
    }
}
