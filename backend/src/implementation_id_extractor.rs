use serde::Deserialize;
use actix_web::{FromRequest, HttpRequest, dev, error::ErrorInternalServerError};
use futures_util::future::{err, ok, Ready};

#[derive(Debug, Deserialize)]
pub struct ImplementationId(pub u64);

/// Returns the implementation_id from a request automatically in the route handlers
impl FromRequest for ImplementationId {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        match req.headers().get("implementation-id") {
            Some(implementation_id) => ok(ImplementationId(implementation_id.to_str().unwrap().parse::<u64>().unwrap())),
            None => err(ErrorInternalServerError("Server error parsing implementation-id")),
        }

    }
}
