use crate::posts::model::{Post, PostRequest};
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::MySqlPool;
use crate::auth::decode_jwt;

#[post("/api/subforums/{id}/posts")]
async fn post_post(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    post: web::Json<PostRequest>,
    auth: BearerAuth,
) -> impl Responder {
    if let Ok(user_id) = decode_jwt(auth.token()) {
        if user_id == post.user_id {
            let result = Post::create(id, post.into_inner(), pool.get_ref()).await;
            return match result {
                Ok(post) => HttpResponse::Ok().json(post),
                _ => HttpResponse::BadRequest().body("Error trying to create new post"),
            };
        }
    }

    HttpResponse::Forbidden().body("Invalid token")
}

#[get("/api/subforums/{id}/posts")]
async fn get_posts(
    web::Path(id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    auth: BearerAuth,
) -> impl Responder {
    if let Ok(_) = decode_jwt(auth.token()) {
        let result = Post::get_all(id, pool.get_ref()).await;
        return match result {
            Ok(posts) => HttpResponse::Ok().json(posts),
            _ => HttpResponse::BadRequest().body("Error trying to retrieve all posts"),
        };
    }

    HttpResponse::Forbidden().body("Invalid token")
}

//#[get("/api/posts/{id}")]
#[get("/api/forums/{forum_id}/subforums/{subforum_id}/posts/{post_id}")]
async fn get_post(
    web::Path(post_id): web::Path<u64>,
    pool: web::Data<MySqlPool>,
    auth: BearerAuth,
) -> impl Responder {
    if let Ok(_) = decode_jwt(auth.token()) {
        let result = Post::get_one(post_id, pool.get_ref()).await;
        return match result {
            Ok(post) => HttpResponse::Ok().json(post),
            _ => HttpResponse::BadRequest().body("Error trying to get post"),
        };
    }

    HttpResponse::Forbidden().body("Invalid token")
}

#[get("api/ping")]
async fn ping() -> impl Responder {
    println!("yeet");
    HttpResponse::Ok()
}
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(post_post);
    cfg.service(ping);
    cfg.service(get_posts);
}
