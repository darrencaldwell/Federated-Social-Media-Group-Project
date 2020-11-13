use super::comments;
use actix_web::{web, get, post, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::MySqlPool;
use crate::auth::decode_jwt;

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
async fn post_comment(web::Path(id): web::Path<u64>,
                      pool: web::Data<MySqlPool>,
                      auth: BearerAuth,
                      comment: web::Json::<comments::CommentRequest>
) -> impl Responder {
    if let Ok(user_id) = decode_jwt(auth.token()) {
        if user_id == comment.user_id {
            return match comments::insert_comment(id, comment.into_inner(), &pool).await {
                Ok(comments) => HttpResponse::Ok().json(comments),
                Err(_) => HttpResponse::InternalServerError().body(""),
            };
        }
    }

    HttpResponse::Forbidden().body("Invalid token")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_comment);
    cfg.service(get_comments);
    cfg.service(post_comment);
}
