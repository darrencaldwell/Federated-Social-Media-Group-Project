use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, web::Data};
use actix_service::{Service, Transform};
use super::util;

use anyhow::Result;
use futures::future::{ok, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use openssl::pkey::{PKey, Private};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct ResponseSign;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for ResponseSign
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ResponseSignMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ResponseSignMiddleware { service })
    }
}

pub struct ResponseSignMiddleware<S> {
    service: S,
}

impl<S, B> Service for ResponseSignMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            // local paths don't need signed, and these ones in particular won't have an associated
            // user-id with the request, so ignore them. This includes requests for our key
            // TODO: we may still want to sign responses for the api key, but we need to figure out
            // what to do if they dont want to send a user id for it? we might require that they
            // do.
            if res.request().path() == "/api/users/login"
                || res.request().path() == "/api/users/register"
                || res.request().path() == "/api/key" {
                return Ok(res)
            };

            let req = res.request().clone();
            let req_headers = req.headers();
            let req_method = req.method().as_str();
            let req_path = req.path();
            let key_pair = req.app_data::<Data<PKey<Private>>>().unwrap().clone();
            // modifies the mutable res headers with the signature.
            util::sign_signature(res.headers_mut(), &req_headers, &req_method, &req_path, &key_pair).unwrap();
            Ok(res)
        })
    }
}
