use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, Done, Row};
use serde::ser::{Serializer, SerializeStruct};
use super::super::request_errors::RequestError;
use bigdecimal::ToPrimitive;
use super::super::voting::{UserVote, parse_mariadb};
use chrono::{DateTime, Utc};
/// Represents a request to POST a post
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRequest {
    pub post_title: String,
    pub post_contents: String,
    pub user_id: String,
    pub username: String,
}

/// Represents a request to modify a post
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPatchRequest {
    pub post_title: String,
    pub post_contents: String,
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
        state.serialize_field("username", &self.username)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("subforumId", &self.subforum_id.to_string())?;
        state.serialize_field("downvotes", &self.downvotes)?;
        state.serialize_field("upvotes", &self.upvotes)?;
        state.serialize_field("_userVotes", &self.user_votes)?;
        state.serialize_field("_links", &self.links)?;
        state.serialize_field("createdTime", &self.created_time)?;
        state.serialize_field("modifiedTime", &self.modified_time)?;
        state.end()
    }
}
#[derive(FromRow)]
pub struct Post {
    pub id: u64,
    pub post_title: String,
    pub post_contents: String,
    pub created_time: i64,
    pub modified_time: i64,
    pub user_id: String,
    pub username: String,
    pub subforum_id: u64,
    pub downvotes: u64,
    pub upvotes: u64,
    pub user_votes: Vec<UserVote>,
    pub links: PostLinks,
}

/// The root of the JSON object, contains all posts
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    #[serde(rename = "_embedded")]
    pub(crate) _embedded: PostList,
}

/// Post List
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PostList {
    pub(crate) post_list: Vec<Post>,
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


/// Modifies an existing post
pub async fn patch(post_id: u64, post: PostPatchRequest, pool: &MySqlPool) -> Result<(), RequestError> {

    let number_modified = sqlx::query!(
        r#"
        UPDATE posts
        SET post_title = ?, post_contents = ?, modified_time = current_timestamp()
        WHERE post_id = ?
        "#,
        post.post_title,
        post.post_contents,
        post_id
    )
        .execute(pool)
        .await?
        .rows_affected();

    if number_modified == 0 {
        Err(RequestError::NotFound(format!("post_id: {} not found", post_id)))
    } else {
        Ok(())
    }
}

/// Deletes an existing post
pub async fn delete(post_id: u64, pool: &MySqlPool) -> Result<(), RequestError> {

    let number_modified = sqlx::query!(
        r#"
        DELETE FROM posts WHERE post_id = ?
        "#,
        post_id
    )
        .execute(pool)
        .await?
        .rows_affected();

    if number_modified == 0 {
        Err(RequestError::NotFound(format!("post_id: {} not found", post_id)))
    } else {
        Ok(())
    }
}

/// Creates / Inserts a post into the database
pub async fn create(subforum_id: u64, post: PostRequest, pool: &MySqlPool, implementation_id: u64) -> Result<Post, RequestError> {
    // pool is used for a transaction, ie a rollbackable operation
    let mut tx = pool.begin().await?;

    // Insert post
    let insert_rec = sqlx::query!(
        r#"
        insert into posts (post_title, user_id, post_contents, subforum_id, implementation_id)
        values( ?, ?, ?, ?, ?)
        RETURNING post_id, created_time
        "#,
        post.post_title,
        post.user_id,
        post.post_contents,
        subforum_id,
        implementation_id
    )
        .fetch_one(pool)
        .await?;

    // Insert username into users table (user_id may already be pressent post GET request via middleware)
    let rows_affected = sqlx::query!(
        r#"
        INSERT INTO users (username, user_id, implementation_id) VALUES(?,?,?)
        ON DUPLICATE KEY UPDATE username = ?
        "#,
        post.username,
        post.user_id,
        implementation_id,
        post.username
    )
        .execute(&mut tx)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(RequestError::NotFound(format!("user_id: {} not found", post.user_id)))
    }

    // Get Forum Id from subforum
    let rec = sqlx::query!(
        r#"
        WITH forum AS (
            SELECT forum_id FROM subforums WHERE subforum_id = ?
        ) SELECT forum_id, CONCAT(i.implementation_url, '/api/users/', ?) AS user_endpoint
        FROM implementations i, forum WHERE implementation_id = ?
        "#,
        subforum_id, &post.user_id, implementation_id
    )
        .fetch_one(pool)
        .await?;

    // commits transaction, ie "cements" it
    tx.commit().await?;

    // return the post as if it was retrieved by a GET
    let created_time: i64 = insert_rec.get::<DateTime<Utc>, usize>(1).timestamp();
    let new_post = Post {
        post_title: post.post_title,
        post_contents: post.post_contents,
        user_id: post.user_id.clone(),
        id: insert_rec.get(0),
        username: post.username.clone(),
        created_time,
        modified_time: created_time, 
        subforum_id,
        downvotes: 0,
        upvotes: 0,
        user_votes: Vec::with_capacity(0),
        links: generate_post_links(insert_rec.get(0), subforum_id, rec.forum_id, rec.user_endpoint.unwrap()),
    };
    Ok(new_post)
}

/// Get all posts within the given subforum
pub async fn get_all(subforum_id: u64, pool: &MySqlPool) -> Result<Embedded> {
    let mut posts = vec![];
    let recs = sqlx::query!(
        r#"
        SELECT
            p.post_id AS "post_id!", post_title AS "post_title!", p.user_id AS "user_id!", u.username, p.created_time, p.modified_time,
            post_contents AS "post_contents!", p.subforum_id AS "subforum_id!", forum_id AS "forum_id!",
            sum(case when pv.is_upvote = 0 then 1 else 0 end) AS "downvotes!",
            sum(case when pv.is_upvote = 1 then 1 else 0 end) AS "upvotes!",
            JSON_OBJECT("_userVotes", JSON_ARRAYAGG(
                JSON_OBJECT("isUpvote", (CASE WHEN is_upvote = 1 then true WHEN is_upvote = 0 THEN false END), "user",
                    CONCAT(i_pv.implementation_url, '/api/users/', pv.user_id)))
            ) AS "user_votes",
            CONCAT(i_p.implementation_url, '/api/users/', p.user_id) AS user_endpoint

        FROM posts p
        INNER JOIN subforums s on p.subforum_id = s.subforum_id
        LEFT JOIN posts_votes pv ON
            p.post_id = pv.post_id
        LEFT JOIN implementations i_pv ON
            pv.implementation_id = i_pv.implementation_id
        LEFT JOIN implementations i_p ON
            p.implementation_id = i_p.implementation_id
        LEFT JOIN users u ON
            p.user_id = u.user_id AND p.implementation_id = u.implementation_id
        WHERE p.subforum_id = ?
        GROUP BY p.post_id
        "#,
        subforum_id
    )
        .fetch_all(pool)
        .await?;

    for rec in recs {
        let user_votes = parse_mariadb(rec.user_votes.clone().unwrap());

        posts.push(Post {
            id: rec.post_id,
            post_title: rec.post_title,
            post_contents: rec.post_contents,
            subforum_id: rec.subforum_id,
            created_time: rec.created_time.unwrap().timestamp(),
            modified_time: rec.modified_time.unwrap().timestamp(),
            // MariaDB returns Decimal from sum, so need to convert
            downvotes: rec.downvotes.to_u64().unwrap(),
            upvotes: rec.upvotes.to_u64().unwrap(),
            user_votes,
            links: generate_post_links(
                rec.post_id,
                rec.subforum_id,
                rec.forum_id,
                rec.user_endpoint.unwrap(),
            ),
            user_id: rec.user_id.to_string(),
            username: rec.username.unwrap().to_string(),
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
        SELECT
            p.post_id AS "post_id!", post_title AS "post_title!", p.user_id AS "user_id!", u.username, p.created_time, p.modified_time,
            post_contents AS "post_contents!", p.subforum_id AS "subforum_id!", forum_id AS "forum_id!",
            sum(case when pv.is_upvote = 0 then 1 else 0 end) AS "downvotes",
            sum(case when pv.is_upvote = 1 then 1 else 0 end) AS "upvotes",
            JSON_OBJECT("_userVotes", JSON_ARRAYAGG(
                JSON_OBJECT("isUpvote", CASE WHEN is_upvote = 1 then true else false end, "user",
                CONCAT(i_pv.implementation_url, '/api/users/', pv.user_id)))
            ) AS "user_votes",
            CONCAT(i_p.implementation_url, '/api/users/', p.user_id) AS user_endpoint
        FROM posts p
        LEFT JOIN subforums ON
            p.subforum_id = subforums.subforum_id
        LEFT JOIN posts_votes pv ON
            pv.post_id = p.post_id
        LEFT JOIN implementations i_pv ON
            pv.implementation_id = i_pv.implementation_id
        LEFT JOIN implementations i_p ON
            p.implementation_id = i_p.implementation_id
        LEFT JOIN users u ON
            p.user_id = u.user_id AND p.implementation_id = u.implementation_id
        WHERE p.post_id = ?
        HAVING p.post_id = ?
        "#,
        id, id
    )
        .fetch_one(pool)
        .await?;

    let user_votes = parse_mariadb(rec.user_votes.clone().unwrap());

    let user_id = rec.user_id;
    let post = Post {
        id: rec.post_id,
        post_title: rec.post_title,
        post_contents: rec.post_contents,
        created_time: rec.created_time.unwrap().timestamp(),
        modified_time: rec.modified_time.unwrap().timestamp(),
        subforum_id: rec.subforum_id,
        // MariaDB returns Decimal from sum, so need to convert
        downvotes: rec.downvotes.unwrap().to_u64().unwrap(),
        upvotes: rec.upvotes.unwrap().to_u64().unwrap(),
        user_votes,
        links: generate_post_links(
            rec.post_id,
            rec.subforum_id,
            rec.forum_id,
            rec.user_endpoint.unwrap(),
        ),
        user_id,
        username: rec.username.unwrap(),
    };
    Ok(post)
}

/// Given parameters, generate the links to meet the protocl specification return JSON
pub(crate) fn generate_post_links(id: u64, subforum_id: u64, forum_id: u64, user_endpoint: String) -> PostLinks {
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
        user: Link { href: user_endpoint },
        comments: Link {
            href: comments_link,
        },
    }
}
