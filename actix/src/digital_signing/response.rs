use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, http::header::HeaderName, http::header::HeaderValue, web::Data};
use actix_service::{Service, Transform};

use anyhow::Result;
use futures::future::{ok, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

use openssl::sign::{Signer, Verifier};
use openssl::rsa::Padding;
use openssl::pkey::{PKey, Private};
use openssl::hash::MessageDigest;
use openssl::base64::{encode_block, decode_block};

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
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            // create signature-input header
            // need sig1=(x, y, z); keyId=x
            let header_string = "sig1=(*request-target, host); keyId=https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/key; alg=hs2019";
            res.headers_mut().insert(HeaderName::from_static("signature-input"), HeaderValue::from_static(header_string));

            // sign
            // TODO: check if we want to not just create our own host header, instead of using the
            // others host
            let string_to_sign = format!("*request-target: {}\nhost: {}", res.request().path(), res.request().headers().get("host").unwrap().to_str().unwrap());

            let key_pair = res.request().app_data::<Data<PKey<Private>>>().unwrap().clone();

            let mut signer = Signer::new(MessageDigest::sha512(), &key_pair).unwrap();
            signer.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
            signer.update(string_to_sign.as_ref()).unwrap();
            let signature = signer.sign_to_vec().unwrap();
            let enc_signature = encode_block(&signature);

            res.headers_mut().insert(HeaderName::from_static("signature"), HeaderValue::from_str(&enc_signature).unwrap());

            Ok(res)
        })
    }
}
