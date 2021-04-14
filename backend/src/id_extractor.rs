use serde::Deserialize;
use actix_web::{FromRequest, HttpRequest, dev, error::ErrorUnauthorized};
use futures_util::future::{err, ok, Ready};

#[derive(Debug, Deserialize)]
pub struct UserId(pub String);

/// Returns the user_id from a request automatically in the route handlers
impl FromRequest for UserId {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        match req.headers().get("user-id") {
            Some(user_id) => ok(UserId(user_id.to_str().unwrap().to_string())),
            None => err(ErrorUnauthorized("No user-id in header")),
        }

    }
}
