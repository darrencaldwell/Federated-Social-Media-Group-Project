use super::model;
use actix_web::{web, get, post, HttpResponse, Responder};
use sqlx::MySqlPool;

#[get("/api/forums/{id}")]
async fn get_forum(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    match model::get_forum(id, &pool).await {
        Ok(forum) => HttpResponse::Ok().json(forum),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[get("/api/forums")]
async fn get_forums(pool: web::Data<MySqlPool>) -> impl Responder {
    match model::get_forums(&pool).await {
        Ok(forums) => HttpResponse::Ok().json(forums),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_forum);
    cfg.service(get_forums);
}
