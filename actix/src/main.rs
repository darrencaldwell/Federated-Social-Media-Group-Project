use actix_web::{App, HttpServer, middleware::Logger, web::Data, dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use actix_service::{Service, Transform};

use anyhow::Result;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::MySqlPool;
use futures::future::{ok, Future, Ready};

use std::env;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

mod posts;
mod users;
mod auth;
mod comments;
mod forums;

// This is ALL boilerplate for a middleware,
// TODO: Move to another file when its done
// https://github.com/casbin-rs/actix-casbin-auth/blob/master/src/middleware.rs
// a link to a helpful implementation of a middleware thats kinda auth

pub struct Auth;

impl<S: 'static, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthMiddleware<S>
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

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut srv = self.service.clone();

        Box::pin(async move {
            let headers = req.headers();
            match headers.get("Authorization") { // if auth is here check for JWT token
                Some(token) => { // TODO: Euan, make this actually check the token,
                    // You'll probably need to read the body to get the user id, heres some middleware that does that
                    // https://github.com/actix/examples/blob/master/middleware/src/read_request_body.rs
                    // good luck
                    println!("{:?}", token);
                    println!("{:?}", req.app_data::<Data<String>>());
                    srv.call(req).await // basically, carry out the request, route it to our functions? etc maybe idk
                },
                None => {
                    // creates an error response and sends it back to the sender
                    return Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
                }
            }
        })
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    // update env with .env file.
    dotenv().unwrap();
    // initiates logger for actix middleware
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // pool used for database connections, gets databse url from env file
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await?;

    HttpServer::new(move || {

        App::new()
            // example of being able to add any data to App
            // Data is functionally a map of Type:Value
            .data("yeehaw".to_string())
            .data(pool.clone())
            // wrap is for "wrapping" middlewaare
            .wrap(Auth)
            .wrap(Logger::default())
            // adds routes from subdirectories
            .configure(posts::init)
            .configure(users::init)
            .configure(comments::init)
            .configure(forums::init)
    })
    .bind("127.0.0.1:21450")?
    .run()
    .await?;

    Ok(())
}
