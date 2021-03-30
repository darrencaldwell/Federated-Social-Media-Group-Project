use super::model;
use actix_web::{web, get, post, patch, delete, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::id_extractor::UserId;
use crate::implementation_id_extractor::ImplementationId;
use super::super::request_errors::RequestError;
use log::info;

#[patch("/api/comments/{id}")]
async fn patch_comment(
        web::Path(id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        comment: web::Json<model::CommentPatchRequest>,
        ImplementationId(implementation_id): ImplementationId,
    ) -> impl Responder {
    // TODO: validate permission to modify comment
    match model::patch(id, comment.into_inner(), pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, patch_comment: {}", implementation_id, e.to_string());
            match e {
                RequestError::NotFound(f) => HttpResponse::NotFound().body(f),
                RequestError::SqlxError(f) => HttpResponse::InternalServerError().body(f.to_string()),
            }
        }
    }
}

#[delete("/api/comments/{id}")]
async fn delete_comment(
        web::Path(id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        ImplementationId(implementation_id): ImplementationId,
    ) -> impl Responder {
    // TODO: validate permission to delete comment
    match model::delete(id, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, delete_comment: {}", implementation_id, e.to_string());
            match e {
                RequestError::NotFound(f) => HttpResponse::NotFound().body(f),
                RequestError::SqlxError(f) => HttpResponse::InternalServerError().body(f.to_string()),
            }
        }
    }
}

#[get("/api/comments/{id}")]
async fn get_comment(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(_user_id): UserId,
) -> impl Responder {
    match model::get_comment(id, &pool).await {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_comment: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/api/comments/{id}/comments")]
async fn get_child_comments(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::get_child_comments(id, &pool).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_child_comments: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/api/posts/{id}/comments")]
async fn get_comments(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::get_comments(id, &pool).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_child_comments: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[post("/api/posts/{id}/comments")]
async fn post_comment(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    comment: web::Json::<model::CommentRequest>,
    UserId(user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    if user_id != comment.user_id { return HttpResponse::Forbidden().finish(); }
    match model::insert_comment(id, user_id, comment.into_inner(), &pool, implementation_id).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, post_comment: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[post("/api/comments/{id}/comments")]
async fn post_child_comment(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    comment: web::Json::<model::CommentRequest>,
    UserId(user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    if user_id != comment.user_id { return HttpResponse::Forbidden().finish(); }
    match model::insert_child_comment(id, user_id, comment.into_inner(), &pool, implementation_id).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, post_child_comment: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(patch_comment);
    cfg.service(delete_comment);
    cfg.service(get_comment);
    cfg.service(get_comments);
    cfg.service(get_child_comments);
    cfg.service(post_comment);
    cfg.service(post_child_comment);
}
