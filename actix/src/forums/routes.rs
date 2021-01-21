use super::forums;
use actix_web::{web, get, post, HttpResponse, HttpRequest, Responder};
use auth_macro::*;
use crate::auth::decode_jwt;
use sqlx::MySqlPool;

#[get("/api/forums/{id}")]
#[protected]
async fn get_forum(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    match forums::get_forum(id, &pool).await {
        Ok(forum) => HttpResponse::Ok().json(forum),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/api/forums")]
#[protected]
async fn post_forum(forum_request: web::Json<forums::PostForumRequest>, pool: web::Data<MySqlPool>) -> impl Responder {
    match forums::post_forum(forum_request.into_inner(), &pool).await {
        Ok(forum) => HttpResponse::Ok().json(forum),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/subforums/{id}")]
#[protected]
async fn get_subforum(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    match forums::get_subforum(id, &pool).await {
        Ok(subforum) => HttpResponse::Ok().json(subforum),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/api/subforums")]
#[protected]
async fn post_subforum(subforum_request: web::Json<forums::PostSubforumRequest>, pool: web::Data<MySqlPool>) -> impl Responder {
    match forums::post_subforum(subforum_request.into_inner(), &pool).await {
        Ok(subforum) => HttpResponse::Ok().json(subforum),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/forums")]
#[protected]
async fn get_forums(pool: web::Data<MySqlPool>) -> impl Responder {
    match forums::get_forums(&pool).await {
        Ok(forums) => HttpResponse::Ok().json(forums),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/forums/{id}/subforums")]
#[protected]
async fn get_subforums(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    match forums::get_subforums(id, &pool).await {
        Ok(subforums) => HttpResponse::Ok().json(subforums),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_forum);
    cfg.service(post_forum);
    cfg.service(get_forums);
    cfg.service(get_subforum);
    cfg.service(post_subforum);
    cfg.service(get_subforums);
}
