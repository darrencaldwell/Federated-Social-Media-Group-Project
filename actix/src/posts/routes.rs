use crate::posts::model::{Post, PostRequest};
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::MySqlPool;

#[post("/api/subforums/{id}/posts")]
async fn post_post(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    post: web::Json<PostRequest>,
) -> impl Responder {
    let result = Post::create(id, post.into_inner(), pool.get_ref()).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        _ => HttpResponse::BadRequest().body("Error trying to create new post"),
    }
}

#[get("/api/subforums/{id}/posts")]
async fn get_posts(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Post::get_all(id, pool.get_ref()).await;
    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        _ => HttpResponse::BadRequest().body("Error trying to retrieve all posts"),
    }
}

#[get("/api/posts/{id}")]
async fn get_post(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Post::get_one(id, pool.get_ref()).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        _ => HttpResponse::BadRequest().body("Error trying to get post"),
    }
}

#[get("api/ping")]
async fn ping() -> impl Responder {
    println!("yeet");
    HttpResponse::Ok()
}
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(post_post);
    cfg.service(ping);
    cfg.service(get_posts);
}
