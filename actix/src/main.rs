use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use anyhow::Result;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::MySqlPool;
use std::env;

use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;

mod posts;
mod users;
mod auth;
mod comments;
mod forums;


// This is ALL boilerplate for a middleware,
// TODO: Move to another file when its done

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for SayHi
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SayHiMiddleware { service })
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service for SayHiMiddleware<S>
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

        // basically this is the code that is run for every request

        println!("Hi from start. You requested: {}", req.path());
        println!("{:?}", req.headers());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            // this is run afterwards, with the response, could be removed
            println!("Hi from response");
            Ok(res)
        })
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().unwrap(); // update env with .env file.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await?;

    HttpServer::new(move || {

    let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(SayHi)
            .data(pool.clone())
            .configure(posts::init)
            .configure(users::init)
            .configure(comments::init)
            .configure(forums::init)
    })
    .workers(1)
    .bind("127.0.0.1:21450")?
    .run()
    .await?;

    Ok(())
}
