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

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_comment);
}
