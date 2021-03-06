use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse, client::Client, web::Data, http::HeaderName};
use actix_service::{Service, Transform};
use sqlx::MySqlPool;

use anyhow::Result;
use futures::future::{ok, Future, Ready};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use openssl::pkey::{PKey, Private};
use log::info;

use super::util;
use super::super::implementations::get_one;

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
    #[allow(clippy::clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut srv = self.service.clone();

        Box::pin(async move {
            // check if request has redirection headers
            if !req.headers().contains_key("redirect") && !req.headers().contains_key("redirect-url") {
                // if not we don't care
                return srv.call(req).await
            }

            let dest_url_complete: String;
            if req.headers().contains_key("redirect") {
                // get URL from querying database
                let pool = req.app_data::<Data<MySqlPool>>().unwrap().clone();
                let id = req.headers().get("redirect").unwrap().to_str().unwrap().parse::<u64>().unwrap();
                if id == 1 {
                    return srv.call(req).await
                }
                let implementation = get_one(id, &pool).await.unwrap();
                dest_url_complete = format!("{}{}", implementation.url, req.path());
            }
            // when we are directly redirecting a url we can just set it.
            else {
                dest_url_complete = req.headers().get("redirect-url").unwrap().to_str().unwrap().to_string();
                if dest_url_complete.starts_with("https://cs3099user-b5") {
                    return srv.call(req).await
                }
            }

            // same path, same headers, sign it, send it off
            let client = req.app_data::<Data<Client>>().unwrap();
            // make request from initial req, copies method and headers
            let mut client_req = client.request(req.method().clone(), &dest_url_complete); // redirect should have url to redirect to "https://yeet.com"
            // add headers from front-end for content-type if exist
            if req.headers().contains_key("content-type") {
                client_req.headers_mut().append(HeaderName::from_static("content-type"),
                                                req.headers().get("content-type").unwrap().clone());
            }
            if req.headers().contains_key("content-length") {
                client_req.headers_mut().append(HeaderName::from_static("content-length"),
                                                req.headers().get("content-length").unwrap().clone());
            }

            // add signature to request
            let req_headers = req.headers();
            let req_method = req.method().as_str().to_lowercase();
            let req_path = req.path();
            let key_pair = req.app_data::<Data<PKey<Private>>>().unwrap().clone();
            util::sign_signature(client_req.headers_mut(), &req_headers, &req_method, &req_path, &key_pair).unwrap();
            info!("Making request to {} With headers: {:#?}",dest_url_complete, client_req.headers());

            // split request to get payload (body)
            let (http_req, payload) = req.into_parts();
            // sends payload with request (useful for POST/PATCH etc)
            let mut response = client_req.send_stream(payload).await.unwrap();

            // verify signature of response, not agreed upon to sign responses in protocol, not in use.
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
