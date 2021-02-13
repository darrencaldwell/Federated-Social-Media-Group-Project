use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use serde::ser::{Serializer, SerializeStruct};

/// Represents a request to POST a post
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRequest {
    pub post_title: String,
    pub post_contents: String,
    pub user_id: String,
}

/// Represents the database record for a given post
impl Serialize for Post {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
            S: Serializer {
        let mut state = serializer.serialize_struct("Post", 6)?;
        state.serialize_field("postTitle", &self.post_title)?;
        state.serialize_field("postContents", &self.post_contents)?;
        state.serialize_field("userId", &self.user_id.to_string())?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("subforumId", &self.subforum_id.to_string())?;
        state.serialize_field("_links", &self.links)?;
        state.end()
    }
}
#[derive(FromRow)]
pub struct Post {
    pub post_title: String,
    pub post_contents: String,
    pub user_id: String,
    pub id: u64,
    pub subforum_id: u64,
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

    let id = sqlx::query!(
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
        id,
        subforum_id,
        links: generate_post_links(id, subforum_id, forum_id.forum_id, &post.user_id),
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
            id: rec.post_id,
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
pub async fn get_one(id: u64, pool: &MySqlPool) -> Result<Post> {
    let rec = sqlx::query!(
        r#"
        SELECT post_id, post_title, UuidFromBin(user_id) AS "user_id: String", post_contents, posts.subforum_id, forum_id FROM posts
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE post_id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    let user_id = rec.user_id.unwrap();
    let post = Post {
        id: rec.post_id,
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
fn generate_post_links(id: u64, subforum_id: u64, forum_id: u64, user_id: &str) -> PostLinks {
    let self_link = format!(
        "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/posts/{}", id
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
        id
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
