use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse, client::Client};
use actix_web::http::{HeaderName, HeaderValue};
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

use super::util;

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

            if (headers.contains_key("Signature") || headers.contains_key("Signature-Input"))
                && headers.contains_key("Authorization") {
                return Ok(req.into_response(HttpResponse::Unauthorized().body("Cannot have both signature and auth headers").into_body()))
            }

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
                match util::check_signature(headers, req.path(), &req.method().as_str().to_lowercase()).await {
                    Ok(_) => return srv.call(req).await,
                    Err(e) =>return Ok(req.into_response(HttpResponse::BadRequest().body(format!("Signature verification: {}", e)).into_body())),
                };
            } else {
                return Ok(req.into_response(HttpResponse::Unauthorized().body("No valid authentication method").into_body()))
            }
        })
    }
}
