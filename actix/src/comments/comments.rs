use sqlx::MySqlPool;
use anyhow::Result;
use serde::{Serialize, Deserialize};

/// Represents a request o post a comment on a post
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct CommentRequest {
    pub comment_content: String,
    pub user_id: String,
    pub username: String,
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

/// Represents an entire comment in the database
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: u64,
    pub comment_content: String,
    pub user_id: String,
    pub username: String,
    pub post_id: u64,
    #[serde(rename = "_links")]
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

fn gen_links(comment_id: u64, parent_comment_id: u64, user_id: &str, post_id: u64, subforum_id: u64, forum_id: u64) -> Links {
    Links {
        _self: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/comments/{}", comment_id) },
        post: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/posts/{}", post_id) },
        parent_comment: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/comments/{}", parent_comment_id) },
        subforum: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/subforums/{}", subforum_id) },
        forum: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums/{}", forum_id) },
        user: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/user/{}", user_id) },
        child_comments: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/comments/{}/comments", comment_id) },
    }
}

/// POSTS a comment given a post to comment on and a comment request
pub async fn insert_comment(post_id: u64,
                            user_id: String,
                            comment_request: CommentRequest,
                            pool: &MySqlPool
) -> Result<Comment> {
    let mut tx = pool.begin().await?;
    let comment_id = sqlx::query!(
        "INSERT INTO comments (comment, user_id, post_id) VALUES (?, UuidToBin(?), ?)",
        comment_request.comment_content,
        comment_request.user_id,
        post_id)
        .execute(&mut tx)
        .await?
        .last_insert_id();

    tx.commit().await?;

    let rec = sqlx::query!(
        r#"SELECT username as "username!", posts.subforum_id as "subforum_id!", forum_id as "forum_id!"
        FROM comments
        LEFT JOIN users on comments.user_id = users.user_id
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE comment_id = ?"#,
        comment_id)
        .fetch_one(pool)
        .await?;

    Ok(Comment {
        id: comment_id,
        comment_content: comment_request.comment_content,
        username: rec.username,
        post_id: post_id,
        links: gen_links(comment_id, comment_id, &user_id, post_id, rec.subforum_id, rec.forum_id),
        user_id: user_id,
    })

}

/// POSTS a comment given a comment to comment on and a comment request
pub async fn insert_child_comment(parent_id: u64,
                            user_id: String,
                            comment_request: CommentRequest,
                            pool: &MySqlPool
) -> Result<Comment> {
    let post_id = sqlx::query!("SELECT post_id FROM comments where comment_id = ?", parent_id)
        .fetch_one(pool)
        .await?
        .post_id;

    let mut tx = pool.begin().await?;
    let comment_id = sqlx::query!(
        "INSERT INTO comments (comment, user_id, post_id, parent_id) VALUES (?, UuidToBin(?), ?, ?)",
        comment_request.comment_content,
        comment_request.user_id,
        post_id,
        parent_id)
        .execute(&mut tx)
        .await?
        .last_insert_id();

    tx.commit().await?;

    let rec = sqlx::query!(
        r#"SELECT username as "username!", posts.subforum_id as "subforum_id!", forum_id as "forum_id!"
        FROM comments
        LEFT JOIN users on comments.user_id = users.user_id
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE comment_id = ?"#,
        comment_id)
        .fetch_one(pool)
        .await?;

    Ok(Comment {
        id: comment_id,
        comment_content: comment_request.comment_content,
        username: rec.username,
        post_id: post_id,
        links: gen_links(comment_id, parent_id, &user_id, post_id, rec.subforum_id, rec.forum_id),
        user_id: user_id,
    })

}

/// Get all comments within a post
pub async fn get_comments(post_id: u64, pool: &MySqlPool) -> Result<Comments> {
    let recs = sqlx::query!(
        r#"SELECT comment, UuidFromBin(comments.user_id) AS "user_id!: String", comment_id,
        posts.subforum_id AS "subforum_id!", subforums.forum_id AS "forum_id!", username AS "username!"
        FROM comments
        LEFT JOIN users on comments.user_id = users.user_id
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE comments.post_id = ?"#,
        post_id)
        .fetch_all(pool)
        .await?;

    let comments: Vec<Comment> = recs.into_iter()
        .map(|rec| {
            Comment {
                id: rec.comment_id,
                comment_content: rec.comment,
                username: rec.username,
                post_id: post_id,
                links: gen_links(rec.comment_id, rec.comment_id, &rec.user_id, post_id,
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
        r#"SELECT comment, UuidFromBin(comments.user_id) AS "user_id!: String", comment_id, comments.post_id,
        posts.subforum_id AS "subforum_id!", subforums.forum_id AS "forum_id!", username AS "username!"
        FROM comments
        LEFT JOIN users on comments.user_id = users.user_id
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE comments.comment_id = ?"#,
        comment_id)
        .fetch_all(pool)
        .await?;

    let comments: Vec<Comment> = recs.into_iter()
        .map(|rec| {
            Comment {
                id: rec.comment_id,
                comment_content: rec.comment,
                username: rec.username,
                post_id: rec.post_id,
                links: gen_links(rec.comment_id, rec.comment_id, &rec.user_id, rec.post_id,
                                 rec.subforum_id, rec.forum_id),
                user_id: rec.user_id,
            }
        }).collect();

    Ok(Comments {
        links: SelfLink {
            _self: Link {
                href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/posts/{}/comments", comments[0].post_id)
            }
        },
        embedded: CommentList { comment_list: comments },
    })

}

/// Get a single comment by it's id
pub async fn get_comment(comment_id: u64, pool: &MySqlPool) -> Result<Comment> {
    let rec = sqlx::query!(
        r#"SELECT comment, UuidFromBin(comments.user_id) AS "user_id!: String", comments.post_id,
        posts.subforum_id AS "subforum_id!", subforums.forum_id AS "forum_id!", username AS "username!"
        FROM comments
        LEFT JOIN users on comments.user_id = users.user_id
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE comment_id = ?"#,
        comment_id)
        .fetch_one(pool)
        .await?;

    Ok(Comment {
        id: comment_id,
        comment_content: rec.comment,
        username: rec.username,
        post_id: rec.post_id,
        links: gen_links(comment_id, comment_id, &rec.user_id, rec.post_id, rec.subforum_id, rec.forum_id),
        user_id: rec.user_id,
    })
}
