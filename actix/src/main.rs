use actix_web::{App, HttpServer, middleware::Logger};

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
use std::path::Path;
use std::fs;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct Key {
    pub key: String,
}

#[get("/api/key")]
async fn get_key(key: web::Data<String>) -> impl Responder {
        HttpResponse::Ok().json(Key {key: key.to_string()})
}

#[actix_web::main]
async fn main() -> Result<()> {
    // update env with .env file.
    dotenv().unwrap();
    // initiates logger for actix middleware
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // pool used for database connections, gets databse url from env file
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await?;

    let pub_path = Path::new("/home/dc228/Documents/uni/cs3099/project-code/actix/src/public_key.pem");
    let public_key = fs::read_to_string(pub_path).unwrap();
    let priv_path = Path::new("/home/dc228/Documents/uni/cs3099/project-code/actix/src/private_key.der");
    let private_key: Vec<u8> = fs::read(priv_path).unwrap();

    HttpServer::new(move || {

        App::new()
            // example of being able to add any data to App
            // Data is functionally a map of Type:Value
            .app_data(public_key.clone())
            .app_data(private_key.clone())
            .data(pool.clone())
            // wrap is for "wrapping" middlewaare
            .wrap(digital_signing::RequestAuth)
            .wrap(digital_signing::ResponseSign)
            .wrap(Logger::default())
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
