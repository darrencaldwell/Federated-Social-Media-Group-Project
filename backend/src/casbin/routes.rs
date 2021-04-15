use actix_web::{web, patch, get, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::{casbin::model, casbin_enforcer::{Role, Object, Action, CasbinData}, id_extractor::UserId, implementation_id_extractor::ImplementationId};

macro_rules! ok_or_server_error {
    ( $x:expr ) => {
        match $x {
            Ok(val) => val,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        }
    };
}

#[get("/local/forums/{forum_id}/subforum/{subforum_id}/permissions")]
async fn get_permissions(
    web::Path((forum_id, subforum_id)): web::Path<(u64, u64)>,
    enforcer: web::Data<CasbinData>,
    UserId(_user_id): UserId,
) -> impl Responder {
    let user =
        ok_or_server_error!(enforcer.get_permissions(&Role::User, forum_id, Some(subforum_id)).await);
    let guest =
        ok_or_server_error!(enforcer.get_permissions(&Role::Guest, forum_id, Some(subforum_id)).await);
    
    let list = model::PermissionsList {
        user,
        guest,
    };

    HttpResponse::Accepted().json(list)
}

#[get("/local/forums/{id}/roles")]
async fn get_roles(
    web::Path(id): web::Path<u64>,
    enforcer: web::Data<CasbinData>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
) -> impl Responder {
    let creators = enforcer.get_roles(id, Role::Creator, &pool).await;
    let moderators = enforcer.get_roles(id, Role::Moderator, &pool).await;
    let users = enforcer.get_roles(id, Role::User, &pool).await;

    let role_list = model::RoleList {
        creators,
        moderators,
        users,
    };

    HttpResponse::Accepted().json(role_list)
}

#[patch("/local/forums/{id}/permissions/users")]
async fn change_role(
    web::Path(id): web::Path<u64>,
    enforcer: web::Data<CasbinData>,
    request: web::Json::<model::UserRequest>,
    UserId(user_id): UserId,
    ImplementationId(imp_id): ImplementationId,
) -> impl Responder {

    let role = match Role::from_str(&request.role) {
        Some(role) => role,
        None => {
            log::info!("Attempt to change to invalid role {}", request.role);
            return HttpResponse::BadRequest().finish()
        }
    };

    let domain = Object::Forum(id);

    let current_roles = enforcer.get_user_role(&request.user, imp_id, &domain).await;
    let requester_roles = enforcer.get_user_role(&user_id, imp_id, &domain).await;

    let allowed =
        if current_roles.iter().any(|r| r == Role::Admin.name()) {
            log::info!("Attempt to change role of admin user");
            false
        } else if requester_roles.iter().any(|r| r == Role::Admin.name() || r == Role::Creator.name()) {
            true
        } else if requester_roles.iter().any(|r| r == Role::Moderator.name()) {
            if current_roles.iter().any(|r| r == Role::User.name() ||
                                            r == Role::Guest.name()) ||
                                            current_roles.is_empty() 
            {
                &request.role == Role::User.name() || request.role == Role::Guest.name()
            } else {
                log::info!("Attempt to change moderator/creator role");
                false
            }
        } else {
            false
    };

    if !allowed { return HttpResponse::Forbidden().finish() }

    if current_roles.len() == 1 {
        let role = Role::from_str(&current_roles[0]).unwrap();
        match enforcer.remove_user_from_group(&request.user, request.imp_id, &role, &domain).await {
            Ok(true) => (),
            _ => return HttpResponse::InternalServerError().finish(),
        }
    }

    return match enforcer.add_user_to_group(&request.user, request.imp_id, &role, &domain).await {
        Ok(true) => HttpResponse::Accepted().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/local/forums/{id}/permissions")]
async fn alter_permissions(
    web::Path(id): web::Path<u64>,
    enforcer: web::Data<CasbinData>,
    request: web::Json::<model::AlterPermissionsRequest>,
    UserId(user_id): UserId,
) -> impl Responder {
    let roles = enforcer.get_user_role(&user_id, 1, &Object::Forum(id)).await;
    if !roles.iter().any(|r| Role::can_lock_post(r)) {
        return HttpResponse::Forbidden().finish();
    }

    let post_effect = if request.can_post { "allow" } else { "deny" };
    let view_effect = if request.can_view { "allow" } else { "deny" };

    let post_policy = vec![
            Role::Guest.name().to_string(),
            Object::Forum(id).name(),
            Object::all_subforums(),
            Action::Write.name().to_string(),
            post_effect.to_string(),
        ];

    let view_policy = vec![
            Role::Guest.name().to_string(),
            Object::Forum(id).name(),
            Object::all_subforums(),
            Action::Read.name().to_string(),
            view_effect.to_string(),
        ];

    let first_success = if !enforcer.has_policy(post_policy.clone()).await {
        let remove_effect = if request.can_post { "deny" } else { "allow" };
        let old = vec![
                Role::Guest.name().to_string(),
                Object::Forum(id).name(),
                Object::all_subforums(),
                Action::Write.name().to_string(),
                remove_effect.to_string(),
            ];

        let val = enforcer.remove_policy(old).await;
        let val = enforcer.add_policy(post_policy).await.and(val);
        val
    } else {
        Ok(true)
    };

    let seccond_success = if !enforcer.has_policy(view_policy.clone()).await {
        let remove_effect = if request.can_view { "deny" } else { "allow" };
        let old = vec![
                Role::Guest.name().to_string(),
                Object::Forum(id).name(),
                Object::all_subforums(),
                Action::Read.name().to_string(),
                remove_effect.to_string(),
            ];

        let val = enforcer.remove_policy(old).await;
        let val = enforcer.add_policy(view_policy).await.and(val);
        val
    } else {
        Ok(true)
    };

    match first_success.and(seccond_success) {
        Ok(true) => HttpResponse::Accepted().finish(),
        Ok(false) => HttpResponse::InternalServerError().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_permissions);
    cfg.service(get_roles);
    cfg.service(change_role);
    cfg.service(alter_permissions);
}
