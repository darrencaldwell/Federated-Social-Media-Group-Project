use actix_web::{get, post, web, HttpResponse, Responder};
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

#[derive(Serialize, Deserialize)]
struct Post {
    postTitle: String,
    postMarkup: String,
    userId: u32,
}

#[get("/text")] // called when receiving a request for get /text
async fn get_text() -> impl Responder {
    // return the current time as a string in the json format text:string
    HttpResponse::Ok().json(MyText {
        text: Utc::now().to_string(),
    })
}

#[post("/api/subforums/{id}/posts")]
async fn post_post(
    web::Path(id): web::Path<u32>,
    pool: web::Data<MySqlPool>,
    post: web::Json<Post>,
) -> impl Responder {
    // create a post instance from the deserialised JSON
    let post_info: Post = post.into_inner();

    // insert into db
    let new_post = sqlx::query_as!(
        Post,
        r#"
    INSERT INTO posts (post_title, user_id, post_contents, subforum_id)
    VALUES( ?, ?, ?, ? )
        "#,
        post_info.postTitle,
        post_info.userId,
        post_info.postMarkup,
        id
    )
    .fetch_one(&**pool)
    .await
    .unwrap();

    Ok(HttpResponse::Ok().json(new_post))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    dotenv().unwrap(); // update env with .env file.
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
    println!("{}", result.username);

    // start server with service get_text to process /text Gets
    HttpServer::new(|| {
        App::new()
            .data(pool.clone())
            .service(get_text)
            .service(post_post)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
