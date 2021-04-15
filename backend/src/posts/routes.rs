use super::model;
use actix_web::{get, post, patch, delete, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::id_extractor::UserId;
use crate::casbin_enforcer::{CasbinData, Action, Object, Role};
use crate::implementation_id_extractor::ImplementationId;
use super::super::request_errors::RequestError;
use log::info;

#[patch("/api/posts/{id}")]
async fn patch_post(
        web::Path(id): web::Path<u64>,
        ImplementationId(implementation_id): ImplementationId,
        pool: web::Data<MySqlPool>,
        post: web::Json<model::PostPatchRequest>,
        UserId(user_id): UserId,
) -> impl Responder {
    let rec = match sqlx::query!(
        r#"SELECT user_id FROM posts WHERE post_id = ?"#
        , id)
        .fetch_one(pool.get_ref())
        .await {
            Ok(rec) => rec,
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if user_id != rec.user_id { return HttpResponse::Forbidden().finish(); }

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

async fn can_delete_post(id: u64, pool: &MySqlPool, enforcer: &CasbinData, user_id: String, imp_id: u64) -> Option<bool> {
    let rec = match sqlx::query!(
        r#"SELECT subforums.forum_id, posts.user_id
        FROM posts LEFT JOIN subforums
        ON posts.subforum_id = subforums.subforum_id
        WHERE post_id = ?"#
        , id)
        .fetch_one(pool)
        .await {
        Err(_) => return None,
        Ok(rec) => rec,
    };

    if user_id == rec.user_id { return Some(true) }

    let forum_id = match rec.forum_id {
        Some(id) => id,
        None => return None,
    };

    match enforcer.enforce(user_id, imp_id, forum_id, Object::Post(id), Action::Delete).await {
        Ok(val) => return Some(val),
        Err(_) => return None,
    };

}

#[delete("/api/posts/{id}")]
async fn delete_post(
        web::Path(id): web::Path<u64>,
        ImplementationId(implementation_id): ImplementationId,
        pool: web::Data<MySqlPool>,
        enforcer: web::Data<CasbinData>,
        UserId(user_id): UserId,
) -> impl Responder {
    match can_delete_post(id, pool.get_ref(), enforcer.get_ref(), user_id, implementation_id).await {
        Some(true) => (),
        Some(false) => return HttpResponse::Forbidden().finish(),
        None => return HttpResponse::InternalServerError().finish(),
    }

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

#[get("/local/posts/{id}/can_delete")]
async fn can_delete_post_route(
        web::Path(id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        enforcer: web::Data<CasbinData>,
        UserId(user_id): UserId,
) -> impl Responder {
    match can_delete_post(id, pool.get_ref(), enforcer.get_ref(), user_id, 1).await {
        Some(true) => HttpResponse::Accepted().finish(),
        Some(false) => HttpResponse::Forbidden().finish(),
        None => HttpResponse::InternalServerError().finish(),
    }
}

async fn can_lock_post(
    post_id: u64,
    user_id: &str,
    imp_id: u64,
    enforcer: &CasbinData,
    pool: &MySqlPool,
) -> Option<(bool, u64)> {
    let rec = sqlx::query!("SELECT forum_id
                                 FROM posts
                                 LEFT JOIN subforums
                                 ON subforums.subforum_id = posts.subforum_id
                                 WHERE post_id = ?",
                                 post_id)
        .fetch_one(pool)
        .await;

    let forum_id = match rec.map(|r| r.forum_id) {
        Ok(Some(forum_id)) => forum_id,
        _ => return None,
    };

    let roles = enforcer.get_user_role(&user_id, imp_id, &Object::Forum(forum_id)).await;

    Some((roles.iter().any(|r| Role::can_lock_post(r)), forum_id))
}

#[get("/local/posts/{id}/can_lock")]
async fn can_lock_post_route(
        web::Path(id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        enforcer: web::Data<CasbinData>,
        UserId(user_id): UserId,
) -> impl Responder {
    let allowed = can_lock_post(id, &user_id, 1, &enforcer, &pool);
    match allowed.await {
        Some((true, _)) => HttpResponse::Accepted().finish(),
        Some((false, _)) => HttpResponse::Forbidden().finish(),
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/local/posts/{id}/lock")]
async fn lock_post(
        web::Path(id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        enforcer: web::Data<CasbinData>,
        post: web::Json<model::LockRequest>,
        UserId(user_id): UserId,
        ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    let allowed = can_lock_post(id, &user_id, implementation_id, &enforcer, &pool);
    match allowed.await {
        Some((true, forum_id)) => {
            for role in &post.roles {
                if let Some(role) = Role::from_str(role) {
                    let _val = enforcer.lock_post(id, forum_id, role).await;
                }
            }

            HttpResponse::Ok().finish()
        },
        Some((false, _)) => HttpResponse::Forbidden().finish(),
        None => HttpResponse::InternalServerError().finish(),
    }
}

async fn can_post_post(forum_id:u64,
                       id: u64,
                       enforcer: &CasbinData,
                       user_id: String,
                       imp_id: u64,
) -> Option<bool> {
    match enforcer.enforce(user_id, imp_id, forum_id, Object::Subforum(id), Action::Write).await {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

#[post("/api/subforums/{id}/posts")]
async fn post_post(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    post: web::Json<model::PostRequest>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(user_id): UserId,
    enforcer: web::Data<CasbinData>,
) -> impl Responder {
    if user_id != post.user_id { return HttpResponse::Forbidden().finish(); }

    let forum_id = match sqlx::query!(
        r#"SELECT forum_id FROM subforums WHERE subforum_id = ?"#
        , id)
        .fetch_one(pool.get_ref())
        .await {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(rec) => rec.forum_id,
    };

    match can_post_post(forum_id, id, enforcer.get_ref(), user_id, implementation_id).await {
        Some(true) => (),
        Some(false) => return HttpResponse::Forbidden().finish(),
        None => return HttpResponse::InternalServerError().finish(),
    };

    let result = model::create(forum_id, id, post.into_inner(), pool.get_ref(), implementation_id).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, post_post: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/local/subforums/{id}/canPost")]
async fn can_post_post_route(
        web::Path(id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        enforcer: web::Data<CasbinData>,
        UserId(user_id): UserId,
) -> impl Responder {
    let forum_id = match sqlx::query!(
        r#"SELECT forum_id FROM subforums WHERE subforum_id = ?"#
        , id)
        .fetch_one(pool.get_ref())
        .await {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(rec) => rec.forum_id,
    };

    match can_post_post(forum_id, id, enforcer.get_ref(), user_id, 1).await {
        Some(true) => HttpResponse::Accepted().finish(),
        Some(false) => HttpResponse::Forbidden().finish(),
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/api/subforums/{id}/posts")]
async fn get_posts(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    enforcer: web::Data<CasbinData>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(user_id): UserId,
) -> impl Responder {
    let forum_id = match sqlx::query!(
        r#"SELECT forum_id FROM subforums WHERE subforum_id = ?"#
        , id)
        .fetch_one(pool.get_ref())
        .await {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(rec) => rec.forum_id,
    };

    match enforcer.enforce(user_id, implementation_id, forum_id, Object::Subforum(id), Action::Read).await {
        Ok(true) => (),
        Ok(false) => return HttpResponse::Forbidden().finish(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

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
    enforcer: web::Data<CasbinData>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(user_id): UserId,
) -> impl Responder {
    let forum_id = match sqlx::query!(
        r#"SELECT subforums.forum_id
        FROM posts LEFT JOIN subforums
        ON posts.subforum_id = subforums.subforum_id
        WHERE post_id = ?"#
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
    cfg.service(can_delete_post_route);
    cfg.service(post_post);
    cfg.service(can_post_post_route);
    cfg.service(lock_post);
    cfg.service(can_lock_post_route);
    cfg.service(get_posts);
    cfg.service(get_post);
}
