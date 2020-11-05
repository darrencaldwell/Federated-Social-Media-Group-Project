use crate::posts::model::{Post, PostRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::MySqlPool;

#[post("/api/subforums/{id}/posts")]
async fn post_post(
    web::Path(id): web::Path<u32>,
    pool: web::Data<MySqlPool>,
    post: web::Json<PostRequest>,
) -> impl Responder {
    let result = Post::create(id, post.into_inner(), pool.get_ref()).await;
    println!("{}", result);
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        _ => HttpResponse::BadRequest().body("Error trying to create new post"),
    }
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok()
}
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(post_post);
    cfg.service(ping);
}
