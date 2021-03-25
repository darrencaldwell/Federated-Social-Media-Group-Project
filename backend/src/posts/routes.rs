use super::model;
use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::id_extractor::UserId;
use crate::implementation_id_extractor::ImplementationId;
use super::super::request_errors::RequestError;
use log::info;

#[patch("/api/posts/{id}")]
async fn patch_post(
        web::Path(id): web::Path<u64>,
        ImplementationId(implementation_id): ImplementationId,
        pool: web::Data<MySqlPool>,
        post: web::Json<model::PostPatchRequest>,
    ) -> impl Responder {
    // TODO: validate permission to modify post
    match model::patch(id, post.into_inner(), pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, patch_post: {}", implementation_id, e.to_string());
            match e {
                RequestError::NotFound(f) => HttpResponse::NotFound().body(f),
                RequestError::SqlxError(f) => HttpResponse::InternalServerError().body(f.to_string()),
            }
        }
    }
}

#[delete("/api/posts/{id}")]
async fn delete_post(
        web::Path(id): web::Path<u64>,
        ImplementationId(implementation_id): ImplementationId,
        pool: web::Data<MySqlPool>,
    ) -> impl Responder {
    // TODO: validate permission to delete post
    match model::delete(id, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, delete_post: {}", implementation_id, e.to_string());
            match e {
                RequestError::NotFound(f) => HttpResponse::NotFound().body(f),
                RequestError::SqlxError(f) => HttpResponse::InternalServerError().body(f.to_string()),
            }
        }
    }
}

#[post("/api/subforums/{id}/posts")]
async fn post_post(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    post: web::Json<model::PostRequest>,
    UserId(user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    if user_id != post.user_id { return HttpResponse::Forbidden().finish(); }
    let result = model::create(id, post.into_inner(), pool.get_ref(), implementation_id).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, post_post: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/api/subforums/{id}/posts")]
async fn get_posts(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    let result = model::get_all(id, pool.get_ref()).await;
    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_posts: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/api/posts/{id}")]
async fn get_post(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    let result = model::get_one(id, pool.get_ref()).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_post: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(patch_post);
    cfg.service(delete_post);
    cfg.service(post_post);
    cfg.service(get_posts);
    cfg.service(get_post);
}
