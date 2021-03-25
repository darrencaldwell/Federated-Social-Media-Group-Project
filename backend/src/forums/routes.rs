use super::model;
use crate::implementation_id_extractor::ImplementationId;
use actix_web::{web, get, post, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::id_extractor::UserId;
use log::info;

#[get("/api/forums/{id}")]
async fn get_forum(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::get_forum(id, &pool).await {
        Ok(forum) => HttpResponse::Ok().json(forum),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_forum_id: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[post("/api/forums")]
async fn post_forum(
    forum_request: web::Json<model::PostForumRequest>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::post_forum(forum_request.into_inner(), &pool).await {
        Ok(forum) => HttpResponse::Ok().json(forum),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, post_forum: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/api/subforums/{id}")]
async fn get_subforum(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::get_subforum(id, &pool).await {
        Ok(subforum) => HttpResponse::Ok().json(subforum),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_subforum: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[post("/api/forums/{id}/subforums")]
async fn post_subforum(
    web::Path(id): web::Path<u64>,
    subforum_request: web::Json<model::PostSubforumRequest>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::post_subforum(id, subforum_request.into_inner(), &pool).await {
        Ok(subforum) => HttpResponse::Ok().json(subforum),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, post_subforum: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/api/forums")]
async fn get_forums(
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::get_forums(&pool).await {
        Ok(forums) => HttpResponse::Ok().json(forums),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_forums: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/api/forums/{id}/subforums")]
async fn get_subforums(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::get_subforums(id, &pool).await {
        Ok(subforums) => HttpResponse::Ok().json(subforums),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_subforums: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
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
