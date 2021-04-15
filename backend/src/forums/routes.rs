use crate::implementation_id_extractor::ImplementationId;
use actix_web::{web, delete, get, post, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::forums::model;
use crate::id_extractor::UserId;
use crate::casbin_enforcer::{Object, Role, Action, CasbinData};
use log::info;

#[get("/api/forums/{id}")]
async fn get_forum(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    enforcer: web::Data<CasbinData>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(user_id): UserId,
) -> impl Responder {
    match enforcer.enforce(user_id, implementation_id, id, Object::Forum(id), Action::Read).await {
        Ok(true) => (),
        Ok(false) => return HttpResponse::Forbidden().finish(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

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
    enforcer: web::Data<CasbinData>,
    UserId(user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::post_forum(forum_request.into_inner(), &user_id, implementation_id, &pool, &enforcer).await {
        Ok(forum) => HttpResponse::Ok().json(forum),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, post_forum: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[delete("/local/forums/{id}")]
async fn delete_forum(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    enforcer: web::Data<CasbinData>,
    UserId(user_id): UserId,
) -> impl Responder {
    let roles = enforcer.get_user_role(&user_id, 1, &Object::Forum(id)).await;

    if !roles.iter().any(|r| r == Role::Admin.name() || r == Role::Creator.name()) {
        return HttpResponse::Forbidden().finish()
    }

    match model::delete_forum(id, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}


#[delete("/local/subforums/{id}")]
async fn delete_subforum(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    enforcer: web::Data<CasbinData>,
    UserId(user_id): UserId,
) -> impl Responder {
    let res = sqlx::query!("SELECT forum_id FROM subforums
                                 WHERE subforum_id = ?",
                                 id)
        .fetch_one(pool.get_ref())
        .await;

    let _forum_id = match res {
        Ok(rec) => rec.forum_id,
        Err(_) => return HttpResponse::BadRequest().body("No such subforum"),
    };

    let roles = enforcer.get_user_role(&user_id, 1, &Object::Forum(id)).await;

    if !roles.iter().any(|r| r == Role::Admin.name() || r == Role::Creator.name() || r == Role::Moderator.name()) {
        return HttpResponse::Forbidden().finish()
    }

    match model::delete_subforum(id, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/api/subforums/{id}")]
async fn get_subforum(
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
    enforcer: web::Data<CasbinData>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(user_id): UserId,
) -> impl Responder {
    match enforcer.enforce(user_id, implementation_id, id, Object::Forum(id), Action::Write).await {
        Ok(true) => (),
        Ok(false) => return HttpResponse::Forbidden().finish(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

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
    enforcer: web::Data<CasbinData>,
    UserId(user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match enforcer.enforce(user_id.clone(), implementation_id, id, Object::Forum(id), Action::Read).await {
        Ok(true) => (),
        Ok(false) => return HttpResponse::Forbidden().finish(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    match model::get_subforums(id, user_id, implementation_id, &pool, &enforcer).await {
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
    cfg.service(delete_forum);
    cfg.service(delete_subforum);
}
