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
            .wrap(digital_signing::RequestAuth)
            .wrap(digital_signing::ResponseSign)
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
