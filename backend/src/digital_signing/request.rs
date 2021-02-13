use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use actix_web::http::{HeaderName, HeaderValue};
use actix_service::{Service, Transform};

use anyhow::Result;
use futures::future::{ok, Future, Ready};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use log::info;

use super::util;

#[derive(Debug)]
struct SignatureInput {
    alg: String,
    created: u64,
    expires: u64,
    key_id: String,
    covered_content: String,
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
    #[allow(clippy::type_complexity)]
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
                let error = "Cannot have both signature and auth headers";
                info!("Req Rejected: {}",error);
                return Ok(req.into_response(HttpResponse::Unauthorized().body(error).into_body()))
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
                srv.call(req).await
            } else if headers.contains_key("Signature") && headers.contains_key("Signature-Input") {
                match util::check_signature(headers, req.path(), &req.method().as_str().to_lowercase()).await {
                    Ok(_) => srv.call(req).await,
                    Err(e) => {
                        let error = format!("Signature verification: {}\nWith signature-input: {:?}\nWith signature {:?}",
                                            e,
                                            req.headers().get("signature-input").unwrap(),
                                            req.headers().get("signature").unwrap());
                        info!("Request Rejected: {}", error);
                        Ok(req.into_response(HttpResponse::BadRequest().body(error).into_body()))
                    }
                }
            } else {
                let error = "No valid authentication method";
                info!("Req Rejected: {}", error);
                Ok(req.into_response(HttpResponse::Unauthorized().body(error).into_body()))
            }
        })
    }
}
