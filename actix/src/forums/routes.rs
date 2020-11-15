use super::model;
use actix_web::{web, get, post, HttpResponse, HttpRequest, Responder};
use auth_macro::*;
use crate::auth::decode_jwt;
use sqlx::MySqlPool;

#[get("/api/forums/{id}")]
#[protected]
async fn get_forum(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    match model::get_forum(id, &pool).await {
        Ok(forum) => HttpResponse::Ok().json(forum),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[get("/api/subforums/{id}")]
#[protected]
async fn get_subforum(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    match model::get_subforum(id, &pool).await {
        Ok(subforum) => HttpResponse::Ok().json(subforum),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[get("/api/forums")]
#[protected]
async fn get_forums(pool: web::Data<MySqlPool>) -> impl Responder {
    match model::get_forums(&pool).await {
        Ok(forums) => HttpResponse::Ok().json(forums),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[get("/api/forums/{id}/subforums")]
#[protected]
async fn get_subforums(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    match model::get_subforums(id, &pool).await {
        Ok(subforums) => HttpResponse::Ok().json(subforums),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_forum);
    cfg.service(get_forums);
    cfg.service(get_subforum);
    cfg.service(get_subforums);
}
