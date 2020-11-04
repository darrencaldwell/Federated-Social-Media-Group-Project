use actix_web::{get, HttpResponse, Responder};
use chrono::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::env;

#[derive(Serialize, Deserialize)]
struct MyText {
    // Struct used as effectively a JSON skeleton for sending as a response
    text: String,
}

#[get("/text")] // called when receiving a request for get /text
async fn get_text() -> impl Responder {
    // return the current time as a string in the json format text:string
    HttpResponse::Ok().json(MyText {
        text: Utc::now().to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    dotenv().unwrap();
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await;
    let pool = match pool {
        Ok(pool) => pool,
        Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error)),
    };
    let result = sqlx::query!("SELECT * FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("{:?}", result);
    println!("{}", result.user_id);

    // start server with service get_text to process /text Gets
    HttpServer::new(|| App::new().service(get_text))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
