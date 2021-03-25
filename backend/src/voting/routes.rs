use super::model;
use actix_web::{put, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::id_extractor::UserId;
use crate::implementation_id_extractor::ImplementationId;
use super::super::request_errors::RequestError;
use log::info;

// update a users vote to post or comment
#[put("/api/posts/{post_id}/vote")]
async fn put_post_vote(
        web::Path(post_id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        vote: web::Json<model::VoteRequest>,
        UserId(user_id): UserId,
        ImplementationId(implementation_id): ImplementationId,
    ) -> impl Responder {
    match model::put_post_vote(post_id, user_id, implementation_id, vote.into_inner(), pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, put_post_vote: {}", implementation_id, e.to_string());
            match e {
                RequestError::NotFound(f) => HttpResponse::NotFound().body(f),
                RequestError::SqlxError(f) => HttpResponse::InternalServerError().body(f.to_string()),
            }
        }
    }
}
#[put("/api/comments/{post_id}/vote")]
async fn put_comment_vote(
        web::Path(comment_id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        vote: web::Json<model::VoteRequest>,
        UserId(user_id): UserId,
        ImplementationId(implementation_id): ImplementationId,
    ) -> impl Responder {
    match model::put_comment_vote(comment_id, user_id, implementation_id, vote.into_inner(), pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, put_comment_vote: {}", implementation_id, e.to_string());
            match e {
                RequestError::NotFound(f) => HttpResponse::NotFound().body(f),
                RequestError::SqlxError(f) => HttpResponse::InternalServerError().body(f.to_string()),
            }
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(put_post_vote);
    cfg.service(put_comment_vote);
}
