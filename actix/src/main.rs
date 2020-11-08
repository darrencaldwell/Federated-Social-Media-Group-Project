use actix_web::middleware::Logger;
use anyhow::Result;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::MySqlPool;
use std::env;

mod posts;

#[actix_web::main]
async fn main() -> Result<()> {
    use actix_web::{App, HttpServer};
    dotenv().unwrap(); // update env with .env file.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let pool = MySqlPool::connect(&env::var("DATABASE_URL").unwrap()).await?;

    // start server with service get_text to process /text Gets
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(posts::init)
    })
    .workers(1)
    .bind("127.0.0.1:5000")?
    .run()
    .await?;

    Ok(())
}
