use super::comments;
use actix_web::{web, get, post, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::id_extractor::UserId;

#[get("/api/comments/{id}")]
async fn get_comment(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
) -> impl Responder {
    match comments::get_comment(id, &pool).await {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/posts/{id}/comments")]
async fn get_comments(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
) -> impl Responder {
    match comments::get_comments(id, &pool).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/api/posts/{id}/comments")]
async fn post_comment(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    comment: web::Json::<comments::CommentRequest>,
    UserId(user_id): UserId
) -> impl Responder {
    if user_id != comment.user_id { return HttpResponse::Forbidden().finish(); }
    match comments::insert_comment(id, comment.into_inner(), &pool).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_comment);
    cfg.service(get_comments);
    cfg.service(post_comment);
}
