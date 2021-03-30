use sqlx::{MySqlPool, Done, Row};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct};
use bigdecimal::ToPrimitive;
use chrono::{DateTime, Utc};
use super::super::request_errors::RequestError;
use super::super::voting::{UserVote, parse_mariadb};

/// Represents a request to post a comment on a post
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct CommentRequest {
    pub comment_content: String,
    pub user_id: String,
    pub username: String,
}
/// Represents a request to modify a comment
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentPatchRequest {
    pub comment_content: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfLink {
    pub _self: Link,
}

#[derive(Serialize)]
pub struct Link {
    pub href: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub _self: Link,
    pub post: Link,
    pub parent_comment: Link,
    pub subforum: Link,
    pub forum: Link,
    pub user: Link,
    pub child_comments: Link,
}

impl Serialize for Comment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("Comment", 6)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("commentContent", &self.comment_content)?;
        state.serialize_field("userId", &self.user_id)?;
        state.serialize_field("username", &self.username)?;
        state.serialize_field("postId", &self.post_id.to_string())?;
        state.serialize_field("downvotes", &self.downvotes)?;
        state.serialize_field("upvotes", &self.upvotes)?;
        state.serialize_field("_userVotes", &self.user_votes)?;
        state.serialize_field("_links", &self.links)?;
        state.serialize_field("createdTime", &self.created_time)?;
        state.serialize_field("modifiedTime", &self.modified_time)?;
        state.end()
    }
}
/// Represents an entire comment in the database
pub struct Comment {
    pub id: u64,
    pub comment_content: String,
    pub user_id: String,
    pub username: String,
    pub post_id: u64,
    pub downvotes: u64,
    pub upvotes: u64,
    pub user_votes: Vec<UserVote>,
    pub created_time: i64,
    pub modified_time: i64,
    pub links: Links,
}

/// Root of list of [Comment]
#[derive(Serialize)]
pub struct Comments {
    #[serde(rename = "_embedded")]
    pub embedded: CommentList,
    #[serde(rename = "_links")]
    pub links: SelfLink,
}

/// List of [Comment]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentList {
    pub comment_list: Vec<Comment>,
}

pub(crate) fn gen_links(comment_id: u64, parent_comment_id: u64, user_endpoint: String, post_id: u64, subforum_id: u64, forum_id: u64) -> Links {
    Links {
        _self: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/comments/{}", comment_id) },
        post: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/posts/{}", post_id) },
        parent_comment: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/comments/{}", parent_comment_id) },
        subforum: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/subforums/{}", subforum_id) },
        forum: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums/{}", forum_id) },
        user: Link { href: user_endpoint },
        child_comments: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/comments/{}/comments", comment_id) },
    }
}

/// POSTS a comment given a post to comment on and a comment request
pub async fn insert_comment(post_id: u64,
                            user_id: String,
                            comment_request: CommentRequest,
                            pool: &MySqlPool,
                            implementation_id: u64
) -> Result<Comment, RequestError> {
    let mut tx = pool.begin().await?;
    let insert_rec = sqlx::query!(
        "
        INSERT INTO comments 
        (comment, user_id, post_id, implementation_id) VALUES (?, ?, ?, ?)
        RETURNING comment_id, created_time 
        ",
        comment_request.comment_content,
        comment_request.user_id,
        post_id,
        implementation_id)
        .fetch_one(pool)
        .await?;

    let comment_id = insert_rec.get(0);
    let created_time: i64 = insert_rec.get::<DateTime<Utc>, usize>(1).timestamp();

    // Insert username into users table (user_id may already be pressent post GET request via middleware)
    let rows_affected = sqlx::query!(
        r#"
        INSERT INTO users (username, user_id, implementation_id) VALUES(?,?,?)
        ON DUPLICATE KEY UPDATE username = ?
        "#,
        comment_request.username,
        comment_request.user_id,
        implementation_id,
        comment_request.username
    )
        .execute(&mut tx)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(RequestError::NotFound(format!("user_id: {} not found", comment_request.user_id)))
    }

    tx.commit().await?;

    let rec = sqlx::query!(
        r#"SELECT posts.subforum_id as "subforum_id!", forum_id as "forum_id!",
            CONCAT(i.implementation_url, '/api/users/', ?) AS user_endpoint
        FROM comments
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        LEFT JOIN implementations i ON
            comments.implementation_id = i.implementation_id
        WHERE comment_id = ?"#,
        comment_id, comment_id)
        .fetch_one(pool)
        .await?;

    Ok(Comment {
        created_time,
        modified_time: created_time,
        id: comment_id,
        comment_content: comment_request.comment_content,
        username: comment_request.username,
        post_id,
        downvotes: 0,
        upvotes: 0,
        user_votes: Vec::with_capacity(0),
        // this is a root comment so it is it's own parent
        links: gen_links(comment_id, comment_id, rec.user_endpoint.unwrap(), post_id, rec.subforum_id, rec.forum_id),
        user_id,
    })

}

/// POSTS a comment given a comment to comment on and a comment request
pub async fn insert_child_comment(parent_id: u64,
                                  user_id: String,
                                  comment_request: CommentRequest,
                                  pool: &MySqlPool,
                                  implementation_id: u64
) -> Result<Comment, RequestError> {
    let post_id = sqlx::query!("SELECT post_id FROM comments where comment_id = ?", parent_id)
        .fetch_one(pool)
        .await?
        .post_id;

    let mut tx = pool.begin().await?;
    let insert_rec = sqlx::query!(
        "INSERT INTO comments (comment, user_id, post_id, parent_id, implementation_id) VALUES (?, ?, ?, ?, ?)
            RETURNING comment_id, created_time
        ",
        comment_request.comment_content,
        comment_request.user_id,
        post_id,
        parent_id,
        implementation_id)
        .fetch_one(pool)
        .await?;

    let comment_id = insert_rec.get(0);
    let created_time: i64 = insert_rec.get::<DateTime<Utc>, usize>(1).timestamp();

    // Insert username into users table (user_id may already be pressent post GET request via middleware)
    let rows_affected = sqlx::query!(
        r#"
        INSERT INTO users (username, user_id, implementation_id) VALUES(?,?,?)
        ON DUPLICATE KEY UPDATE username = ?
        "#,
        comment_request.username,
        comment_request.user_id,
        implementation_id,
        comment_request.username
    )
        .execute(&mut tx)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(RequestError::NotFound(format!("user_id: {} not found", comment_request.user_id)))
    }
    tx.commit().await?;

    let rec = sqlx::query!(
        r#"SELECT posts.subforum_id as "subforum_id!", forum_id as "forum_id!",
            CONCAT(i.implementation_url, '/api/users/', ?) AS user_endpoint
        FROM comments
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        LEFT JOIN implementations i ON
            comments.implementation_id = i.implementation_id
        WHERE comment_id = ?"#,
        user_id, comment_id)
        .fetch_one(pool)
        .await?;

    Ok(Comment {
        created_time,
        modified_time: created_time,
        id: comment_id,
        comment_content: comment_request.comment_content,
        username: comment_request.username,
        post_id,
        downvotes: 0,
        upvotes: 0,
        user_votes: Vec::with_capacity(0),
        links: gen_links(comment_id, parent_id, rec.user_endpoint.unwrap(), post_id, rec.subforum_id, rec.forum_id),
        user_id,
    })

}

/// Modifies an existing comment
pub async fn patch(comment_id: u64, comment: CommentPatchRequest, pool: &MySqlPool) -> Result<(), RequestError> {

    let number_modified = sqlx::query!(
        r#"
        UPDATE comments
        SET comment = ?, modified_time = current_timestamp()
        WHERE comment_id = ?
        "#,
        comment.comment_content,
        comment_id
    )
        .execute(pool)
        .await?
        .rows_affected();

    if number_modified == 0 {
        Err(RequestError::NotFound(format!("comment_id: {} not found", comment_id)))
    } else {
        Ok(())
    }
}

/// Deletes an existing comment
pub async fn delete(comment_id: u64, pool: &MySqlPool) -> Result<(), RequestError> {

    let number_modified = sqlx::query!(
        r#"
        DELETE FROM comments WHERE comment_id = ?
        "#,
        comment_id
    )
        .execute(pool)
        .await?
        .rows_affected();

    if number_modified == 0 {
        Err(RequestError::NotFound(format!("comment_id: {} not found", comment_id)))
    } else {
        Ok(())
    }
}

/// Get all top level comments within a post
pub async fn get_comments(post_id: u64, pool: &MySqlPool) -> Result<Comments> {
    let recs = sqlx::query!(
        r#"SELECT comment, comments.user_id, comments.comment_id, comments.created_time, comments.modified_time, comments.parent_id,
        posts.subforum_id AS "subforum_id!", subforums.forum_id AS "forum_id!", username AS "username!",
        sum(case when cv.is_upvote = 0 then 1 else 0 end) AS "downvotes!",
        sum(case when cv.is_upvote = 1 then 1 else 0 end) AS "upvotes!",
        JSON_OBJECT("_userVotes", JSON_ARRAYAGG(
            JSON_OBJECT("isUpvote", (CASE WHEN is_upvote = 1 then true WHEN is_upvote = 0 THEN false END), "user",
                CONCAT(i_cv.implementation_url, '/api/users/', cv.user_id)))
        ) AS "user_votes",
        CONCAT(i_c.implementation_url, '/api/users/', comments.user_id) AS user_endpoint
        FROM comments
        LEFT JOIN comments_votes cv ON
            comments.comment_id = cv.comment_id
        LEFT JOIN implementations i_cv ON
            cv.implementation_id = i_cv.implementation_id
        LEFT JOIN implementations i_c ON
            comments.implementation_id = i_c.implementation_id
        LEFT JOIN users on comments.user_id = users.user_id
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE comments.post_id = ? AND parent_id IS NULL
        GROUP BY comments.comment_id
        "#,
        post_id)
        .fetch_all(pool)
        .await?;

    let comments: Vec<Comment> = recs.into_iter()
        .map(|rec| {
            let user_votes = parse_mariadb(rec.user_votes.clone().unwrap());
            Comment {
                created_time: rec.created_time.unwrap().timestamp(),
                modified_time: rec.modified_time.unwrap().timestamp(),
                id: rec.comment_id,
                comment_content: rec.comment,
                username: rec.username,
                post_id,
                // MariaDB returns Decimal from sum, so need to convert
                downvotes: rec.downvotes.to_u64().unwrap(),
                upvotes: rec.upvotes.to_u64().unwrap(),
                user_votes,
                // use parent id if not null, else use comment_id
                links: gen_links(rec.comment_id, rec.parent_id.unwrap_or(rec.comment_id), rec.user_endpoint.unwrap(), post_id,
                                 rec.subforum_id, rec.forum_id),
                user_id: rec.user_id,
            }
        }).collect();

    Ok(Comments {
        embedded: CommentList { comment_list: comments },
        links: SelfLink {
            _self: Link {
                href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/posts/{}/comments", post_id)
            }
        }
    })
}

/// Get all top level child comments of a comment
pub async fn get_child_comments(comment_id: u64, pool: &MySqlPool) -> Result<Comments> {
    let recs = sqlx::query!(
        r#"SELECT comment, comments.user_id, comments.comment_id, comments.post_id, comments.created_time, comments.modified_time,
        comments.parent_id,
        posts.subforum_id AS "subforum_id!", subforums.forum_id AS "forum_id!", username AS "username!",
        sum(case when cv.is_upvote = 0 then 1 else 0 end) AS "downvotes!",
        sum(case when cv.is_upvote = 1 then 1 else 0 end) AS "upvotes!",
        JSON_OBJECT("_userVotes", JSON_ARRAYAGG(
            JSON_OBJECT("isUpvote", (CASE WHEN is_upvote = 1 then true WHEN is_upvote = 0 THEN false END), "user",
                CONCAT(i_cv.implementation_url, '/api/users/', cv.user_id)))
        ) AS "user_votes",
        CONCAT(i_c.implementation_url, '/api/users/', comments.user_id) AS user_endpoint
        FROM comments
        LEFT JOIN comments_votes cv ON
            comments.comment_id = cv.comment_id
        LEFT JOIN implementations i_cv ON
            cv.implementation_id = i_cv.implementation_id
        LEFT JOIN implementations i_c ON
            comments.implementation_id = i_c.implementation_id
        LEFT JOIN users on comments.user_id = users.user_id
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE comments.parent_id = ?
        GROUP BY comment_id
        "#,
        comment_id)
        .fetch_all(pool)
        .await?;

    let comments: Vec<Comment> = recs.into_iter()
        .map(|rec| {
            let user_votes = parse_mariadb(rec.user_votes.clone().unwrap());
            Comment {
                created_time: rec.created_time.unwrap().timestamp(),
                modified_time: rec.modified_time.unwrap().timestamp(),
                id: rec.comment_id,
                comment_content: rec.comment,
                username: rec.username,
                post_id: rec.post_id,
                // MariaDB returns Decimal from sum, so need to convert
                downvotes: rec.downvotes.to_u64().unwrap(),
                upvotes: rec.upvotes.to_u64().unwrap(),
                user_votes,
                // use parent id if not null, else use comment_id
                links: gen_links(rec.comment_id, rec.parent_id.unwrap_or(rec.comment_id), rec.user_endpoint.unwrap(), rec.post_id,
                                 rec.subforum_id, rec.forum_id),
                user_id: rec.user_id,
            }
        }).collect();

    let post_id = if comments.is_empty() {
        sqlx::query!("SELECT post_id FROM comments WHERE comment_id = ?", comment_id).fetch_one(pool).await?.post_id
    } else {
        comments[0].post_id
    };

    Ok(Comments {
        links: SelfLink {
            _self: Link {
                href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/posts/{}/comments", post_id)
            }
        },
        embedded: CommentList { comment_list: comments },
    })

}

/// Get a single comment by it's id
pub async fn get_comment(comment_id: u64, pool: &MySqlPool) -> Result<Comment> {
    let rec = sqlx::query!(
        r#"SELECT comment AS "comment!", comments.user_id as "user_id!", comments.comment_id, comments.post_id AS "post_id!",
        comments.created_time, comments.modified_time,
        posts.subforum_id AS "subforum_id!", subforums.forum_id AS "forum_id!", username AS "username!",
        sum(case when cv.is_upvote = 0 then 1 else 0 end) AS "downvotes!",
        sum(case when cv.is_upvote = 1 then 1 else 0 end) AS "upvotes!",
        JSON_OBJECT("_userVotes", JSON_ARRAYAGG(
            JSON_OBJECT("isUpvote", (CASE WHEN is_upvote = 1 then true WHEN is_upvote = 0 THEN false END), "user",
                CONCAT(i_cv.implementation_url, '/api/users/', cv.user_id)))
        ) AS "user_votes",
        CONCAT(i_c.implementation_url, '/api/users/', comments.user_id) AS user_endpoint
        FROM comments
        LEFT JOIN comments_votes cv ON
            comments.comment_id = cv.comment_id
        LEFT JOIN implementations i_cv ON
            cv.implementation_id = i_cv.implementation_id
        LEFT JOIN implementations i_c ON
            comments.implementation_id = i_c.implementation_id
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        LEFT JOIN users on comments.user_id = users.user_id
        WHERE comments.comment_id = ?
        HAVING comments.comment_id = ?
        "#,
        comment_id, comment_id)
        .fetch_one(pool)
        .await?;

    let user_votes = parse_mariadb(rec.user_votes.clone().unwrap());
    Ok(Comment {
        created_time: rec.created_time.unwrap().timestamp(),
        modified_time: rec.modified_time.unwrap().timestamp(),
        id: comment_id,
        comment_content: rec.comment,
        username: rec.username,
        post_id: rec.post_id,
        // MariaDB returns Decimal from sum, so need to convert
        downvotes: rec.downvotes.to_u64().unwrap(),
        upvotes: rec.upvotes.to_u64().unwrap(),
        user_votes,
        links: gen_links(comment_id, comment_id, rec.user_endpoint.unwrap(), rec.post_id, rec.subforum_id, rec.forum_id),
        user_id: rec.user_id,
    })
}
