use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

use sqlx::{FromRow, MySqlPool};

// user input
#[derive(Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct PostRequest {
    pub post_title: String,
    pub post_markup: String,
    pub user_id: u32,
}

// database record
#[derive(Serialize, FromRow)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Post {
    pub post_title: String,
    pub post_markup: String,
    pub user_id: u32,
    pub post_id: u64,
    pub subforum_id: u32,
}

impl Responder for Post {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and get content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl Post {
    pub async fn create(id: u32, post: PostRequest, pool: &MySqlPool) -> Result<Post> {
        let mut tx = pool.begin().await?;

        // insert into db
        let post_id = sqlx::query!(
            r#"
    insert into posts (post_title, user_id, post_contents, subforum_id)
    values( ?, ?, ?, ? )
        "#,
            post.post_title,
            post.user_id,
            post.post_markup,
            id
        )
        .execute(&mut tx)
        .await?
        .last_insert_id();

        tx.commit().await?;
        let new_post = Post {
            post_title: post.post_title,
            post_markup: post.post_markup,
            user_id: post.user_id,
            post_id: post_id,
            subforum_id: id,
        };
        Ok(new_post)
    }
}
