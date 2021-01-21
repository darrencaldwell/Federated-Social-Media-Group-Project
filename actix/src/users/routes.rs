use super::user;
use crate::auth::{self, decode_jwt};
use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};
use sqlx::MySqlPool;
use auth_macro::*;

#[post("/api/users/register")]
async fn register(post: web::Json<user::UserRegisterRequest>, pool: web::Data<MySqlPool>) -> impl Responder {
    let post = post.into_inner();
    let user = user::register(post.username, post.password, &pool).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/api/users/login")]
async fn login(post: web::Json<user::UserLoginRequest>, pool: web::Data<MySqlPool>) -> impl Responder {
    let result = user::verify(&post.username, &post.password, &pool).await;

    let user_id = match result {
        Ok(result) => result,
        Err(_) => return HttpResponse::Forbidden().body(""),
    };

    println!("after verify: {}", user_id);
    let token = match auth::encode_jwt(user_id.clone(), post.username.clone()) {
        Ok(token) => token,
        Err(_) => return HttpResponse::Forbidden().body(""),
    };

    let res = user::LoginResponse {
        user_id,
        username: post.username.clone(),
        token,
    };

    HttpResponse::Ok().json(res)
}

#[get("/api/users/{id}")]
#[protected]
async fn get_user(web::Path(id): web::Path<String>, pool: web::Data<MySqlPool>) -> impl Responder {
    match user::get_user(id, &pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[get("/api/users")]
#[protected]
async fn get_users(pool: web::Data<MySqlPool>) -> impl Responder {
    match user::get_users(&pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(get_users);
    cfg.service(get_user);
}
