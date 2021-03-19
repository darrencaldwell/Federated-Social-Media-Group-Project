use super::model;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::id_extractor::UserId;
use crate::implementation_id_extractor::ImplementationId;
use super::super::request_errors::RequestError;

// update a users vote to post or comment
#[put("/api/posts/{post_id}/vote")]
async fn put_post_vote(
        web::Path(post_id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        vote: web::Json<model::VoteRequest>,
        UserId(user_id): UserId,
        ImplementationId(implementation_id): ImplementationId,
    ) -> impl Responder {
    match model::put_vote(post_id, user_id, implementation_id, vote.into_inner(), pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            RequestError::NotFound(f) => HttpResponse::NotFound().body(f),
            RequestError::SqlxError(f) => HttpResponse::InternalServerError().body(f.to_string()),
        }
    }
}

//#[put("/local/comments/{comment_id}/vote")]

/// In the context of a user, get the votes of all posts in a subforum
#[get("/local/subforums/{subforum_id}/posts/vote")]
async fn get_posts_votes(
    web::Path(subforum_id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    UserId(user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    let result = model::get_posts_votes(subforum_id, user_id, implementation_id, pool.get_ref()).await;
    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
// top level comments
//#[get("/local/comments/{post_id}/vote")]
// nested comments
// #[get("/local/comments/{comment_id}/comments/vote")]




// get all posts / comments a user has upvoted
//#[get("/local/users/{user_id}/posts/upvote")]
//#[get("/local/users/{user_id}/comments/upvote")]

// get all posts / comments a user has downvoted
//#[get("/local/users/{user_id}/posts/downvote")]
//#[get("/local/users/{user_id}/comments/downvote")]
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(put_post_vote);
    cfg.service(get_posts_votes);
}
