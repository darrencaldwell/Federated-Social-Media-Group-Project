use super::user;
use crate::auth;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Serialize, Deserialize)]
struct UserRequest {
    username: String,
    password: String,
}

// i want to move this to users euan u meanie >:(
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub user_id: u64,
    pub username: String,
    pub token: String,
}

#[post("/api/users/register")]
async fn register(post: web::Json<UserRequest>, pool: web::Data<MySqlPool>) -> impl Responder {
    let post = post.into_inner();
    let user = user::register(post.username, post.password, &pool).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::BadRequest().body("Error registering user"),
    }
}

#[post("/api/users/login")]
async fn login(post: web::Json<UserRequest>, pool: web::Data<MySqlPool>) -> impl Responder {
    let result = user::verify(&post.username, &post.password, &pool).await;

    let user_id = match result {
        Ok(result) => result,
        Err(_) => return HttpResponse::Forbidden().body(""),
    };

    let token = match auth::encode_jwt(user_id, post.username.clone()) {
        Ok(token) => token,
        Err(_) => return HttpResponse::Forbidden().body(""),
    };

    let res = LoginResponse {
        user_id,
        username: post.username.clone(),
        token,
    };

    HttpResponse::Ok().json(res)
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
        Ok(user) => user.to_string(),
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
