use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, http::header::HeaderName, http::header::HeaderValue};
use actix_service::{Service, Transform};

use anyhow::Result;
use futures::future::{ok, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

use base64;
use ring::{
    rand,
    signature::{self, KeyPair},
};

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

            // Create an `RsaKeyPair` from the DER-encoded bytes. This example uses
            // a 2048-bit key, but larger keys are also supported.
            let private_key_der = res.request().app_data::<Vec<u8>>().unwrap();
            let key_pair = signature::RsaKeyPair::from_der(&private_key_der).unwrap();

            // Sign the message "hello, world", using PKCS#1 v1.5 padding and the
            // SHA256 digest algorithm.
            let rng = rand::SystemRandom::new();
            let mut signature = vec![0; key_pair.public_modulus_len()];
            key_pair.sign(&signature::RSA_PKCS1_SHA512, &rng, string_to_sign.as_ref(), &mut signature).unwrap();
            let enc_signature = base64::encode(signature);

            res.headers_mut().insert(HeaderName::from_static("signature"), HeaderValue::from_str(&enc_signature).unwrap());

            println!("Hi from response: {:?}", res.headers());
            Ok(res)
        })
    }
}
