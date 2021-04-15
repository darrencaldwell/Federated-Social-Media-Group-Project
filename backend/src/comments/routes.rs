use actix_web::{web, get, post, patch, delete, HttpResponse, Responder};
use sqlx::MySqlPool;
use log::info;

use crate::{
    id_extractor::UserId,
    implementation_id_extractor::ImplementationId,
    casbin_enforcer::{Object, Action, CasbinData},
    comments::model,
    request_errors::RequestError,
};

#[patch("/api/comments/{id}")]
async fn patch_comment(
        web::Path(id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        comment: web::Json<model::CommentPatchRequest>,
        ImplementationId(implementation_id): ImplementationId,
        UserId(user_id): UserId,
) -> impl Responder {
    let rec = match sqlx::query!("SELECT user_id FROM comments WHERE comment_id = ?", id)
        .fetch_one(pool.get_ref())
        .await {
            Ok(rec) => rec,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if user_id != rec.user_id { return HttpResponse::Forbidden().finish() }

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
        enforcer: web::Data<CasbinData>,
        ImplementationId(implementation_id): ImplementationId,
        UserId(user_id): UserId,
) -> impl Responder {
    let rec = match sqlx::query!(
        r#"SELECT subforums.forum_id, comments.user_id
        FROM comments
        LEFT JOIN posts ON comments.post_id = posts.post_id
        LEFT JOIN subforums ON posts.subforum_id = subforums.subforum_id
        WHERE comment_id = ?"#
        , id)
        .fetch_one(pool.get_ref())
        .await {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(rec) => rec,
    };

    if user_id != rec.user_id {
        let forum_id = match rec.forum_id {
            Some(id) => id,
            None => return HttpResponse::InternalServerError().finish(),
        };

        match enforcer.enforce(user_id, implementation_id, forum_id, Object::Comment, Action::Delete).await {
            Ok(true) => (),
            Ok(false) => return HttpResponse::Forbidden().finish(),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        };
    }

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
    enforcer: web::Data<CasbinData>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(user_id): UserId,
) -> impl Responder {
    let rec = match sqlx::query!(
        r#"SELECT subforums.forum_id, subforums.subforum_id
        FROM comments
        LEFT JOIN posts ON comments.post_id = posts.post_id
        LEFT JOIN subforums ON posts.subforum_id = subforums.subforum_id
        WHERE comment_id = ?"#
        , id)
        .fetch_one(pool.get_ref())
        .await {
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            Ok(rec) => rec,
    };

    let forum_id = match rec.forum_id {
        Some(id) => id,
        None => return HttpResponse::InternalServerError().finish(),
    };

    let subforum_id = match rec.subforum_id {
        Some(id) => id,
        None => return HttpResponse::InternalServerError().finish(),
    };

    match enforcer.enforce(user_id, implementation_id, forum_id, Object::Subforum(subforum_id), Action::Read).await {
        Ok(true) => (),
        Ok(false) => return HttpResponse::Forbidden().finish(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

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
    enforcer: web::Data<CasbinData>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(user_id): UserId,
) -> impl Responder {
    let forum_id = match sqlx::query!(
        r#"SELECT subforums.forum_id
        FROM comments
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums ON posts.subforum_id = subforums.subforum_id
        WHERE comment_id = ?"#
        , id)
        .fetch_one(pool.get_ref())
        .await
        .map(|rec| rec.forum_id) {
            Ok(Some(forum_id)) => forum_id,
            Ok(None) => return HttpResponse::InternalServerError().body("Database failure"),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    match enforcer.enforce(user_id, implementation_id, forum_id, Object::Subforum(id), Action::Read).await {
        Ok(true) => (),
        Ok(false) => return HttpResponse::Forbidden().finish(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

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
    enforcer: web::Data<CasbinData>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(user_id): UserId,
) -> impl Responder {
    let (subforum_id, forum_id) = match sqlx::query!(
        r#"SELECT subforums.subforum_id, subforums.forum_id FROM posts
        LEFT JOIN subforums ON posts.subforum_id = subforums.subforum_id
        WHERE posts.post_id = ?"#
        , id)
        .fetch_one(pool.get_ref())
        .await
        .map(|rec| (rec.subforum_id, rec.forum_id)) {
            Ok((Some(sub_id), Some(forum_id))) => (sub_id, forum_id),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            _ => return HttpResponse::InternalServerError().body("Database error"),
    };

    match enforcer.enforce(user_id, implementation_id, forum_id, Object::Subforum(subforum_id), Action::Read).await {
        Ok(true) => (),
        Ok(false) => return HttpResponse::Forbidden().finish(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

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
    enforcer: web::Data<CasbinData>,
    comment: web::Json::<model::CommentRequest>,
    UserId(user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    if user_id != comment.user_id { return HttpResponse::Forbidden().finish(); }
    let forum_id = match sqlx::query!(
        r#"SELECT subforums.forum_id FROM posts
        LEFT JOIN subforums ON subforums.subforum_id = posts.subforum_id
        WHERE post_id = ?"#
        , id)
        .fetch_one(pool.get_ref())
        .await
        .map(|rec| rec.forum_id) {
            Ok(Some(forum_id)) => forum_id,
            Ok(None) => return HttpResponse::InternalServerError().body("Database error"),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    match enforcer.enforce(user_id.clone(), implementation_id, forum_id, Object::Post(id), Action::Write).await {
        Ok(true) => (),
        Ok(false) => return HttpResponse::Forbidden().finish(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

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
    enforcer: web::Data<CasbinData>,
    comment: web::Json::<model::CommentRequest>,
    UserId(user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    if user_id != comment.user_id { return HttpResponse::Forbidden().finish(); }
    let (forum_id, post_id) = match sqlx::query!(
        r#"SELECT subforums.forum_id, comments.post_id FROM comments
        LEFT JOIN posts ON comments.post_id = posts.post_id
        LEFT JOIN subforums ON subforums.subforum_id = posts.subforum_id
        WHERE comment_id = ?"#
        , id)
        .fetch_one(pool.get_ref())
        .await
        .map(|rec| (rec.forum_id, rec.post_id)) {
            Ok((Some(forum_id), post_id)) => (forum_id, post_id),
            Ok((None, _)) => return HttpResponse::InternalServerError().body("Database error"),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    match enforcer.enforce(user_id.clone(), implementation_id, forum_id, Object::Post(post_id), Action::Write).await {
        Ok(true) => (),
        Ok(false) => return HttpResponse::Forbidden().finish(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

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
