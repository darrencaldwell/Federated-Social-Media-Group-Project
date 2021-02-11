use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse, client::Client, web::Data};
use actix_web::HttpMessage;
use actix_web::dev::ResponseBody;
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
use openssl::hash::MessageDigest;
use openssl::base64::{decode_block};
use openssl::pkey::{PKey, Private};

use super::util;

// This is ALL boilerplate for a middleware,
// TODO: Move to another file when its done
// https://github.com/casbin-rs/actix-casbin-auth/blob/master/src/middleware.rs
// a link to a helpful implementation of a middleware thats kinda auth

pub struct Keys {
    pub private: Vec<u8>,
    pub public: Vec<u8>,
}

pub struct ProxyReq;

impl<S: 'static, B> Transform<S> for ProxyReq
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ProxyReqMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ProxyReqMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct ProxyReqMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for ProxyReqMiddleware<S>
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
            // check if request has header "redirect"
            if !req.headers().contains_key("redirect") {
                // if not we don't care
                return srv.call(req).await
            }

            // make request!
            // same path, same headers, sign it, send it off

            let client = Client::default();
            let dest_url = req.headers().get("redirect").unwrap().to_str().unwrap();
            let dest_url_complete = format!("{}{}", dest_url, req.path());
            println!("{}",dest_url_complete);
            // make request from initial req, copies method and headers
            let mut client_req = client.request_from(dest_url_complete, req.head()); // redirect should have url to redirect to "https://yeet.com"

            // add signature to request
            let req_headers = req.headers();
            let req_method = req.method().as_str().to_lowercase();
            let req_path = req.path();
            let key_pair = req.app_data::<Data<PKey<Private>>>().unwrap().clone();
            util::sign_signature(client_req.headers_mut(), &req_headers, &req_method, &req_path, &key_pair).unwrap();

            // split request to get payload (body)
            let (http_req, payload) = req.into_parts();
            // sends payload with request (useful for POST/PATCH etc)
            let mut response = client_req.send_stream(payload).await.unwrap();

            // verify signature of response
            /*
            let signature_verf = util::check_signature(response.headers(), &http_req.path(), &http_req.method().as_str().to_lowercase()).await;
            if signature_verf.is_err() {
                let e = signature_verf.unwrap_err();
                return Ok(ServiceResponse::new(http_req, HttpResponse::BadRequest().body(format!("Signature verification: {}", e)).into_body()));
            };
            */
            // uses up the future to get the body so we can make a new response
            let body = response.body().await.unwrap();

            let mut new_response = HttpResponse::build(response.status());

            // if we have a body, we want to know what type it is
            if response.headers().contains_key("content-type") {
                new_response.set_header("content-type", response.headers().get("content-type").unwrap().as_ref());
            }

            let new_res = ServiceResponse::new(
                http_req,
                new_response.body(body).into_body(),
                );

            Ok(new_res)
        })
    }
}