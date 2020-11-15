use crate::posts::model::{Post, PostRequest};
use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};
use sqlx::MySqlPool;
use crate::auth::decode_jwt;
use auth_macro::*;

#[post("/api/subforums/{id}/posts")]
#[auth_user(post.user_id)]
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
#[protected]
async fn get_posts(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    let result = Post::get_all(id, pool.get_ref()).await;
    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        _ => HttpResponse::BadRequest().body("Error trying to retrieve all posts"),
    }
}

//#[get("/api/posts/{id}")]
#[get("/api/forums/{forum_id}/subforums/{subforum_id}/posts/{post_id}")]
#[protected]
async fn get_post(
    web::Path(post_id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    let result = Post::get_one(post_id, pool.get_ref()).await;
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
    cfg.service(get_post);
}
