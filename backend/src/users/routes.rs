use super::model;
use crate::auth;
use actix_web::{get, post, patch, web, HttpResponse, Responder, Error, http::StatusCode, dev::HttpResponseBuilder};
use sqlx::MySqlPool;
use crate::id_extractor::UserId;
use crate::implementation_id_extractor::ImplementationId;
use actix_multipart as mp;
use futures_util::stream::StreamExt;
use log::info;
use crate::request_errors::RequestError;
use crate::casbin_enforcer::CasbinData;

#[patch("/local/users/{id}")]
async fn patch_user(
    web::Path(id): web::Path<String>,
    pool: web::Data<MySqlPool>,
    user: web::Json<model::UserPatchRequest>
) -> impl Responder {
    match model::patch_user(id, user.into_inner(), &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            info!("ROUTE ERROR: patch_user_bio: {}", e.to_string());
            match e {
                RequestError::NotFound(f) => HttpResponse::NotFound().body(f),
                RequestError::SqlxError(f) => HttpResponse::InternalServerError().body(f.to_string()),
            }
        }
    }
}

#[post("/local/users/{id}/profilepicture")]
async fn profile_picture(mut payload: mp::Multipart, pool: web::Data<MySqlPool>, web::Path(id): web::Path<String>) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        if !field.content_type().to_string().starts_with("image/") {
            return Ok(HttpResponse::BadRequest()
                .body("Profile picutre must be image format."));
        }

        let mut vec: Vec<u8> = Vec::new();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            vec.append(&mut chunk?.as_ref().to_vec());
        }
        // add to database
        sqlx::query!(
            r#"
            UPDATE users
            SET profile_picture = ?
            WHERE user_id = ?
            "#,
            vec,
            id
        )
            .execute(pool.as_ref())
            .await.unwrap();
    }
    Ok(HttpResponse::Ok().into())
}

#[get("/api/users/{id}/profilepicture")]
async fn get_profile_picture(pool: web::Data<MySqlPool>, web::Path(id): web::Path<String>) -> Result<HttpResponse, Error> {
    let img = sqlx::query!(
        r#"
        SELECT profile_picture as pp
        FROM users
        WHERE user_id = ?
        "#,
        id
    )
        .fetch_one(pool.as_ref())
        .await.unwrap()
        .pp.unwrap();
    let res = HttpResponseBuilder::new(StatusCode::OK).content_type("image").body(img);
    Ok(res)
}

#[post("/api/users/register")]
async fn register(post: web::Json<model::UserRegisterRequest>, pool: web::Data<MySqlPool>) -> impl Responder {
    let post = post.into_inner();
    let user = model::register(post.username, post.password, post.first_name, post.last_name, post.email, &pool).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/api/users/login")]
async fn login(post: web::Json<model::UserLoginRequest>, pool: web::Data<MySqlPool>) -> impl Responder {
    // very login by checking username and password matches the hash of the password in the
    // database
    let result = model::verify(&post.email, &post.password, &pool).await;

    let (user_id, username) = match result {
        Ok(result) => result,
        Err(_) => return HttpResponse::Forbidden().body(""),
    };

    println!("after verify: {}", user_id);
    // if successful, returns the token in the response
    let token = match auth::encode_jwt(user_id.clone(), post.email.clone()) {
        Ok(token) => token,
        Err(_) => return HttpResponse::Forbidden().body(""),
    };

    let exp = match auth::decode_jwt_expr(&token) {
        Ok(exp) => exp,
        Err(_) => return HttpResponse::Forbidden().body(""),
    };

    let res = model::LoginResponse {
        user_id,
        username,
        token,
        exp,
    };

    HttpResponse::Ok().json(res)
}

#[get("/api/users/{id}")]
async fn get_user(
    web::Path(id): web::Path<String>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::get_user(id, implementation_id, &pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_user: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/local/forums/{id}/userIdentity/{username}")]
async fn user_search(
    path: web::Path<(u64, String)>,
    pool: web::Data<MySqlPool>,
    enforcer: web::Data<CasbinData>,
    UserId(_user_id): UserId,
) -> impl Responder {
    let (id, username) = path.into_inner();
    match model::search_users(username, id, &enforcer, &pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/api/users")]
async fn get_users(
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
    ImplementationId(implementation_id): ImplementationId,
) -> impl Responder {
    match model::get_users(&pool).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_users: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/local/users/{id}")]
async fn get_account(
    web::Path(id): web::Path<String>,
    pool: web::Data<MySqlPool>,
    UserId(_user_id): UserId,
) -> impl Responder {
    match model::get_account(id, &pool).await {
        Ok(account) => HttpResponse::Ok().json(account),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/users/{id}/posts")]
async fn get_user_posts(
    web::Path(id): web::Path<String>,
    pool: web::Data<MySqlPool>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(_user_id): UserId,
) -> impl Responder {
    match model::get_user_posts(id, pool.get_ref()).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_user_posts: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}


#[get("/api/users/{id}/comments")]
async fn get_user_comments(
    web::Path(id): web::Path<String>,
    pool: web::Data<MySqlPool>,
    ImplementationId(implementation_id): ImplementationId,
    UserId(_user_id): UserId,
) -> impl Responder {
    match model::get_user_comments(id, &pool).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => {
            info!("ROUTE ERROR: impl_id: {}, get_user_comments: {}", implementation_id, e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(patch_user);
    cfg.service(get_users);
    cfg.service(get_user);
    cfg.service(get_account);
    cfg.service(get_user_posts);
    cfg.service(get_user_comments);
    cfg.service(profile_picture);
    cfg.service(get_profile_picture);
    cfg.service(user_search);
}

