use super::model;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use super::super::request_errors::RequestError;
use log::info;

#[put("/local/implementations/{id}")]
async fn put_implementation(
        web::Path(id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
        implementation: web::Json<model::ImplementationRequest>,
    ) -> impl Responder {
    // TODO: validate permission to modify implementation
    match model::put(id, implementation.into_inner(), pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
           info!("ROUTE ERROR: put_implementation: {}", e.to_string());
           match e {
               RequestError::NotFound(f) => HttpResponse::NotFound().body(f),
               RequestError::SqlxError(f) => HttpResponse::InternalServerError().body(f.to_string()),
           }
        }
    }
}
#[delete("/local/implementations/{id}")]
async fn delete_implementation(
        web::Path(id): web::Path<u64>,
        pool: web::Data<MySqlPool>,
    ) -> impl Responder {
    // TODO: validate permission to delete implementation
    match model::delete(id, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
           info!("ROUTE ERROR: delete_implementation: {}", e.to_string());
           match e {
               RequestError::NotFound(f) => HttpResponse::NotFound().body(f),
               RequestError::SqlxError(f) => HttpResponse::InternalServerError().body(f.to_string()),
           }
        }
    }
}
#[post("/local/implementations")]
async fn post_implementation(
    pool: web::Data<MySqlPool>,
    implementation: web::Json<model::ImplementationRequest>,
) -> impl Responder {
    let result = model::post(implementation.into_inner(), pool.get_ref()).await;
    match result {
        Ok(implementation) => HttpResponse::Ok().json(implementation),
        Err(e) => {
           info!("ROUTE ERROR: post_implementation: {}", e.to_string());
           HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
#[get("/local/implementations")]
async fn get_implementations(
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    let result = model::get_all(pool.get_ref()).await;
    match result {
        Ok(implementations) => HttpResponse::Ok().json(implementations),
        Err(e) => {
           info!("ROUTE ERROR: get_implementations: {}", e.to_string());
           HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[get("/local/implementations/{id}")]
async fn get_one_implementation(
   web::Path(id): web::Path<u64>,
   pool: web::Data<MySqlPool>,
) -> impl Responder {
    let result = model::get_one(id, pool.get_ref()).await;
    match result {
        Ok(implementation) => HttpResponse::Ok().json(implementation),
        Err(e) => {
           info!("ROUTE ERROR: get_one_implementation: {}", e.to_string());
           HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
   cfg.service(put_implementation);
   cfg.service(delete_implementation);
   cfg.service(post_implementation);
   cfg.service(get_implementations);
   cfg.service(get_one_implementation);
}
