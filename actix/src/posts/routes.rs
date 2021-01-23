use super::posts;
use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};
use sqlx::MySqlPool;
use crate::auth::decode_jwt;
use auth_macro::*;

#[post("/api/subforums/{id}/posts")]
#[auth_user(post.user_id)]
async fn post_post(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    post: web::Json<posts::PostRequest>,
) -> impl Responder {
    let result = posts::create(id, post.into_inner(), pool.get_ref()).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/subforums/{id}/posts")]
//#[protected] TODO: doesn't like this? Double bound parameter?
async fn get_posts(
    req: HttpRequest,
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    let result = posts::get_all(id, pool.get_ref()).await;
    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/forums/{forum_id}/subforums/{subforum_id}/posts/{post_id}")]
#[protected]
async fn get_post(
    web::Path(post_id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    let result = posts::get_one(post_id, pool.get_ref()).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(post_post);
    cfg.service(get_posts);
    cfg.service(get_post);
}
