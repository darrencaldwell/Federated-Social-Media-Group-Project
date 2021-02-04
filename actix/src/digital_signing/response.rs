use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, http::header::HeaderName, http::header::HeaderValue, web::Data, http::header::Date};
use actix_service::{Service, Transform};

use anyhow::Result;
use futures::future::{ok, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::SystemTime;

use openssl::sign::{Signer};
use openssl::rsa::Padding;
use openssl::pkey::{PKey, Private};
use openssl::hash::MessageDigest;
use openssl::base64::{encode_block};

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
            println!("{:?}", res.request().path());
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

            // create signature-input header
            // need sig1=(x, y, z); keyId=x
            let header_string = "sig1=(*request-target, date, user-id); keyId=https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/key; alg=RSASSA-PSS-SHA512";
            res.headers_mut().insert(HeaderName::from_static("signature-input"), HeaderValue::from_static(header_string));
            let date = Date(SystemTime::now().into());
            res.headers_mut().insert(HeaderName::from_static("date"), HeaderValue::from_str(&date.to_string()).unwrap());

            let request_headers = res.request().headers().clone(); // needed for borrowing shenanigans
            let user = request_headers.get("user-id").unwrap().to_str().unwrap();
            res.headers_mut().insert(HeaderName::from_static("user-id"), HeaderValue::from_str(user).unwrap());

            // sign
            // TODO: check if we want to not just create our own host header, instead of using the
            // others host
            let string_to_sign = format!("*request-target: {} {}\ndate: {}\nuser-id: {}",
            res.request().method().as_str().to_lowercase(),
            res.request().path(),
            date.to_string(),
            res.request().headers().get("user-id").unwrap().to_str().unwrap());
            println!("{:?}",string_to_sign);

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
