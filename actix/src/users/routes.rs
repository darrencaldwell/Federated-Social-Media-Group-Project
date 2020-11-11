use super::user;
use crate::auth;
use actix_web::{web, get, post, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::MySqlPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserRequest {
    username: String,
    password: String,
}

#[post("/api/users/register")]
async fn register(post: web::Json::<UserRequest>, pool: web::Data<MySqlPool>) -> impl Responder {
    let post = post.into_inner();
    let user = user::register(post.username, post.password, &pool).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::BadRequest().body("Error registering user"),
    }
}

#[post("/api/users/login")]
async fn login(post: web::Json::<UserRequest>, pool: web::Data<MySqlPool>) -> impl Responder {
    let valid = user::verify(&post.username, &post.password, &pool).await;

    if !valid {
        return HttpResponse::Forbidden().body("");
    }

    match auth::encode_jwt(post.username.clone()) {
        Ok(token) => HttpResponse::Ok().content_type("plain/text").body(token),
        Err(_) => HttpResponse::Forbidden().body(""),
    }
}

#[get("/api/users/{id}")]
async fn get_user(web::Path(id): web::Path<u64>, pool: web::Data<MySqlPool>) -> impl Responder {
    match user::get_user(id, &pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[get("/api/users")]
async fn get_users(pool: web::Data<MySqlPool>) -> impl Responder {
    match user::get_users(&pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body(""),
    }
}

#[get("/test")]
async fn test(auth: BearerAuth) -> String {
    let s = auth.token();
    
    match auth::decode_jwt(s) {
        Ok(user) => user,
        Err(e) => format!("error: {}", e),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(test);
    cfg.service(get_users);
    cfg.service(get_user);
}
