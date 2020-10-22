use actix_web::{get, HttpResponse, Responder};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyText {
    // Struct used as effectively a JSON skeleton for sending as a response
    text: String,
}

#[get("/text")] // called when receiving a request for get /text
async fn get_text() -> impl Responder {
    // return the current time as a string in the json format text:string
    HttpResponse::Ok().json(MyText {
        text: Utc::now().to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    // start server with service get_text to process /text Gets
    HttpServer::new(|| App::new().service(get_text))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
