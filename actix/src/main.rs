use actix_web::{App, HttpServer, middleware};

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
mod digital_signing;
mod id_extractor;

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
        let public_key_pem = key_pair.public_key_to_pem().unwrap();
        HttpResponse::Ok().json(Key{ key: std::str::from_utf8(&public_key_pem).unwrap().to_string() })
}

#[actix_web::main]
async fn main() -> Result<()> {
    // update env with .env file.
    dotenv().unwrap();
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
            // wrap is for "wrapping" middlewaare
            .wrap(middleware::Compress::default())
            .wrap(digital_signing::RequestAuth)
            .wrap(digital_signing::ResponseSign)
            .wrap(middleware::Logger::default())
            // adds routes from subdirectories
            .service(get_key)
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
