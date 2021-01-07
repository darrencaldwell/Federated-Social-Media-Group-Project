use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;
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

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().unwrap(); // update env with .env file.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await?;

    let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

    // start server with service get_text to process /text Gets
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
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
