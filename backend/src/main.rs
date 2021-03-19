use actix_web::{App, HttpServer, middleware, web::PathConfig, error, client::Client};

use anyhow::Result;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::MySqlPool;
use std::env;

mod posts;
mod users;
mod auth;
mod comments;
mod forums;
mod middlewares;
mod id_extractor;
mod implementation_id_extractor;
mod request_errors;
mod implementations;
mod voting;

use serde::{Serialize, Deserialize};
use actix_web::{web, Responder, get, HttpResponse};
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct Key {
    pub key: String,
}

#[get("/api/key")]
async fn get_key(key_pair: web::Data<PKey<Private>>) -> impl Responder {
        let public_key_pem = key_pair.rsa().unwrap().public_key_to_pem_pkcs1().unwrap();
        HttpResponse::Ok().json(Key{ key: std::str::from_utf8(&public_key_pem).unwrap().to_string() })
}

#[actix_web::main]
async fn main() -> Result<()> {
    // update env with .env file.
    dotenv().unwrap();
    std::env::set_var("RUST_LOG", "info,sqlx=error");
    // initiates logger for actix middleware
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // pool used for database connections, gets databse url from env file
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await?;

    let key_pair = Rsa::generate(2048).unwrap();
    let key_pair: PKey<Private> = PKey::from_rsa(key_pair).unwrap();

    HttpServer::new(move || {

        App::new()
            // example of being able to add any data to App
            // Data is functionally a map of Type:Value
            .data(key_pair.clone())
            .data(pool.clone())
            // construct a client for each worker
            .data(Client::default())
            // configures the error that is returned when an unparsable var is used in the path,
            // eg an id that is not a u64
            .app_data(PathConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                        err,
                        HttpResponse::BadRequest().body("Unparsable id in path, only number id's are supported."),
                    )
                    .into()
            }))
            .wrap(middlewares::ProxyReq)
            .wrap(middlewares::ProtectLocal)
            // auth middleware has to be at bottom,
            .wrap(middlewares::ResponseSign)
            .wrap(middlewares::RequestAuth)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            // adds routes from subdirectories
            .service(get_key)
            .configure(posts::init)
            .configure(users::init)
            .configure(comments::init)
            .configure(forums::init)
            .configure(implementations::init)
            .configure(voting::init)
    })
    .bind("127.0.0.1:21450")?
    .workers(4)
    .run()
    .await?;

    Ok(())
}
