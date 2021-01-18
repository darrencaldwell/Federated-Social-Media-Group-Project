use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

use sqlx::{FromRow, MySqlPool};

// user input
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRequest {
    pub post_title: String,
    pub post_markup: String,
    pub user_id: String,
}

// database record
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub post_title: String,
    pub post_markup: String,
    pub user_id: String,
    pub post_id: u64,
    pub subforum_id: u64,
    #[serde(rename = "_links")]
    pub links: PostLinks,
}

// database record
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    #[serde(rename = "_embedded")]
    _embedded: PostList,
}

// database record
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PostList {
    post_list: Vec<Post>,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PostLinks {
    #[serde(rename = "self")]
    _self: Link,
    subforum: Link,
    forum: Link,
    user: Link,
    comments: Link,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
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
    pub async fn create(subforum_id: u64, post: PostRequest, pool: &MySqlPool) -> Result<Post> {
        // i don't know why this is different, the code i copied has tx over pool directly im not
        // sure why.
        let mut tx = pool.begin().await?;

        // insert into db
        let post_id = sqlx::query!(
            r#"
    insert into posts (post_title, user_id, post_markup, subforum_id)
    values( ?, UuidToBin(?), ?, ? )
        "#,
            post.post_title,
            post.user_id,
            post.post_markup,
            subforum_id
        )
        .execute(&mut tx)
        .await?
        .last_insert_id();

        let forum_id = sqlx::query!(
            r#"
            SELECT forum_id FROM subforums WHERE subforum_id = ?
            "#,
            subforum_id
        )
        .fetch_one(pool)
        .await?;

        tx.commit().await?;

        // return the post as if it was retrieved by a GET
        let new_post = Post {
            post_title: post.post_title,
            post_markup: post.post_markup,
            user_id: post.user_id.clone(),
            post_id: post_id,
            subforum_id: subforum_id,
            links: generate_post_links(post_id, subforum_id, forum_id.forum_id, &post.user_id),
        };
        Ok(new_post)
    }

    pub async fn get_all(subforum_id: u64, pool: &MySqlPool) -> Result<Embedded> {
        let mut posts = vec![];
        let recs = sqlx::query!(
            r#"
            SELECT post_id, post_title, UuidFromBin(user_id) AS "user_id: String", post_markup, subforum_id FROM posts WHERE subforum_id = ?
            ORDER BY post_id
            "#,
            subforum_id
        )
        .fetch_all(pool)
        .await?;

        let forum_id = sqlx::query!(
            r#"
            SELECT forum_id FROM subforums WHERE subforum_id = ?
            "#,
            subforum_id
        )
        .fetch_one(pool)
        .await?;

        for rec in recs {
            let user_id = rec.user_id.unwrap();
            posts.push(Post {
                post_id: rec.post_id,
                post_title: rec.post_title,
                post_markup: rec.post_markup,
                subforum_id: rec.subforum_id,
                links: generate_post_links(
                    rec.post_id,
                    rec.subforum_id,
                    forum_id.forum_id,
                    &user_id,
                ),
                user_id,
            });
        }
        let post_list = PostList { post_list: posts };
        let embedded = Embedded {
            _embedded: post_list,
        };
        Ok(embedded)
    }

    pub async fn get_one(post_id: u64, pool: &MySqlPool) -> Result<Post> {
        let rec = sqlx::query!(
            r#"
            SELECT post_id, post_title, UuidFromBin(user_id) AS "user_id: String", post_markup, subforum_id FROM posts WHERE post_id = ?
            "#,
            post_id
        )
        .fetch_one(pool)
        .await?;

        let forum_id = sqlx::query!(
            r#"
            SELECT forum_id FROM subforums WHERE subforum_id = ?
            "#,
            rec.subforum_id
        )
        .fetch_one(pool)
        .await?;

        let user_id = rec.user_id.unwrap();
        let post = Post {
            post_id: rec.post_id,
            post_title: rec.post_title,
            post_markup: rec.post_markup,
            subforum_id: rec.subforum_id,
            links: generate_post_links(
                rec.post_id,
                rec.subforum_id,
                forum_id.forum_id,
                &user_id,
            ),
            user_id,
        };
        Ok(post)
    }
}

fn generate_post_links(post_id: u64, subforum_id: u64, forum_id: u64, user_id: &String) -> PostLinks {
    let self_link = format!(
        "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums/{}/subforums/{}/posts/{}",
        forum_id, subforum_id, post_id
    );

    let subforum_link = format!(
        "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/subforums/{}",
        subforum_id
    );

    let forum_link = format!(
        "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums/{}",
        forum_id
    );

    let user_link = format!(
        "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/users/{}",
        user_id
    );

    let comments_link = format!(
        "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/posts/{}/comments",
        post_id
    );

    PostLinks {
        _self: Link { href: self_link },
        subforum: Link {
            href: subforum_link,
        },
        forum: Link { href: forum_link },
        user: Link { href: user_link },
        comments: Link {
            href: comments_link,
        },
    }
}
