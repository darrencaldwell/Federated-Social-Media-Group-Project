use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySqlPool, Row};

// user input
#[derive(Serialize, Deserialize)]
struct PostRequest {
    pub postTitle: String,
    pub postMarkup: String,
    pub userId: u32,
}

// database record
#[derive(Serialize, FromRow)]
struct Post {
    pub postTitle: String,
    pub postMarkup: String,
    pub userId: u32,
    pub postId: u32,
    pub subforumId: String,
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
        let post = sqlx::query!(
            r#"
    insert into posts (post_title, user_id, post_contents, subforum_id)
    values( ?, ?, ?, ? )
        "#,
            post.postTitle,
            post.userId,
            post.postMarkup,
            id
        )
        .execute(&mut tx)
        .await?
        .last_insert_id();

        tx.commit().await?;
        let new_post = Post {
            postTitle: post.postTitle,
            postMarkup: post.postMarkup,
            userId: post.userId,
            postId: post,
            subforumId: id,
        };
        Ok(new_post)
    }
}
