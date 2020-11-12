use super::comments;
use actix_web::{web, get, post, HttpResponse, Responder};
use sqlx::MySqlPool;

#[get("/api/comments/{id}")]
async fn get_comment(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    match comments::get_comment(id, &pool).await {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[get("/api/posts/{id}/comments")]
async fn get_comments(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    match comments::get_comments(id, &pool).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[post("/api/posts/{id}/comments")]
async fn post_comment(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>,
                      comment: web::Json::<comments::CommentRequest>) -> impl Responder {
    println!("here");
    match comments::insert_comment(id, comment.into_inner(), &pool).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_comment);
    cfg.service(get_comments);
    cfg.service(post_comment);
}