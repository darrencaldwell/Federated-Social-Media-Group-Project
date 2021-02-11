use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

/// Represents a request to POST a post
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRequest {
    pub post_title: String,
    pub post_contents: String,
    pub user_id: String,
}

/// Represents the database record for a given post
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub post_title: String,
    pub post_contents: String,
    pub user_id: String,
    pub post_id: u64,
    pub subforum_id: u64,
    #[serde(rename = "_links")]
    pub links: PostLinks,
}

/// The root of the JSON object, contains all posts
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    #[serde(rename = "_embedded")]
    _embedded: PostList,
}

/// Post List
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PostList {
    post_list: Vec<Post>,
}

/// Contains all of the links for a given post
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

/// Creates / Inserts a post into the database
pub async fn create(subforum_id: u64, post: PostRequest, pool: &MySqlPool) -> Result<Post> {
    // pool is used for a transaction, ie a rollbackable operation
    let mut tx = pool.begin().await?;

    let post_id = sqlx::query!(
        r#"
        insert into posts (post_title, user_id, post_contents, subforum_id)
        values( ?, UuidToBin(?), ?, ? )
        "#,
        post.post_title,
        post.user_id,
        post.post_contents,
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

    // commits transaction, ie "cements" it
    tx.commit().await?;

    // return the post as if it was retrieved by a GET
    let new_post = Post {
        post_title: post.post_title,
        post_contents: post.post_contents,
        user_id: post.user_id.clone(),
        post_id,
        subforum_id,
        links: generate_post_links(post_id, subforum_id, forum_id.forum_id, &post.user_id),
    };
    Ok(new_post)
}

/// Get all posts within the given subforum
pub async fn get_all(subforum_id: u64, pool: &MySqlPool) -> Result<Embedded> {
    let mut posts = vec![];
    let recs = sqlx::query!(
        r#"
        SELECT post_id, post_title, UuidFromBin(user_id) AS "user_id: String", post_contents, posts.subforum_id, forum_id FROM posts
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE posts.subforum_id = ?
        ORDER BY post_id
        "#,
        subforum_id
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        let user_id = rec.user_id.unwrap();
        posts.push(Post {
            post_id: rec.post_id,
            post_title: rec.post_title,
            post_contents: rec.post_contents,
            subforum_id: rec.subforum_id,
            links: generate_post_links(
                rec.post_id,
                rec.subforum_id,
                rec.forum_id.unwrap(),
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

///  Get a single post by its id
pub async fn get_one(post_id: u64, pool: &MySqlPool) -> Result<Post> {
    let rec = sqlx::query!(
        r#"
        SELECT post_id, post_title, UuidFromBin(user_id) AS "user_id: String", post_contents, posts.subforum_id, forum_id FROM posts
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE post_id = ?
        "#,
        post_id
    )
    .fetch_one(pool)
    .await?;

    let user_id = rec.user_id.unwrap();
    let post = Post {
        post_id: rec.post_id,
        post_title: rec.post_title,
        post_contents: rec.post_contents,
        subforum_id: rec.subforum_id,
        links: generate_post_links(
            rec.post_id,
            rec.subforum_id,
            rec.forum_id.unwrap(),
            &user_id,
        ),
        user_id,
    };
    Ok(post)
}

/// Given parameters, generate the links to meet the protocl specification return JSON
fn generate_post_links(post_id: u64, subforum_id: u64, forum_id: u64, user_id: &str) -> PostLinks {
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
