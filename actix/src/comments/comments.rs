use sqlx::MySqlPool;
use anyhow::Result;
use serde::{Serialize, Deserialize};

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
pub struct Comment {
    pub id: u64,
    pub comment_content: String,
    pub user_id: String,
    pub post_id: u64,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub _self: Link,
    pub post: Link,
    pub subforum: Link,
    pub forum: Link,
    pub user: Link,
}

#[derive(Serialize)]
pub struct Link {
    pub href: String,
}

#[derive(Serialize)]
pub struct Comments {
    #[serde(rename = "_embedded")]
    pub embedded: CommentList,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentList {
    pub comment_list: Vec<Comment>,
}

fn gen_links(comment_id: u64, user_id: &String, post_id: u64, subforum_id: u64, forum_id: u64) -> Links {
    Links {
        _self: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/comments/{}", comment_id) },
        post: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/posts/{}", post_id) },
        subforum: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/subforums/{}", subforum_id) },
        forum: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/forums/{}", forum_id) },
        user: Link { href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/user/{}", user_id) },
    }
}

async fn get_subforum(post_id: u64, pool: &MySqlPool) -> Result<u64> {
    Ok(sqlx::query!(
        "SELECT subforum_id FROM posts WHERE post_id = ?",
        post_id)
        .fetch_one(pool)
        .await?
        .subforum_id)
}

async fn get_forum(subforum_id: u64, pool: &MySqlPool) -> Result<u64> {
    Ok(sqlx::query!(
        " SELECT forum_id FROM subforums WHERE subforum_id = ?",
        subforum_id)
        .fetch_one(pool)
        .await?
        .forum_id)
}

pub async fn insert_comment(post_id: u64, comment_request: CommentRequest,
                            pool: &MySqlPool) -> Result<Comment> {
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

    let subforum_id = get_subforum(post_id, pool).await?;
    let forum_id = get_forum(subforum_id, pool).await?;

    Ok(Comment {
        id: comment_id,
        comment_content: comment_request.comment_content,
        post_id,
        links: gen_links(comment_id, &comment_request.user_id, post_id, subforum_id, forum_id),
        user_id: comment_request.user_id,
    })

}

pub async fn get_comments(post_id: u64, pool: &MySqlPool) -> Result<Comments> {
    let recs = sqlx::query!(
        r#"SELECT comment, UuidFromBin(user_id) AS "user_id: String", comment_id FROM comments WHERE post_id = ?"#,
        post_id)
        .fetch_all(pool)
        .await?;

    let subforum_id = get_subforum(post_id, pool).await?;
    let forum_id = get_forum(subforum_id, pool).await?;

    let comments: Vec<Comment> = recs.into_iter()
        .map(|rec| {
            let user_id = rec.user_id.unwrap();
            Comment {
                id: rec.comment_id,
                comment_content: rec.comment,
                post_id,
                links: gen_links(rec.comment_id, &user_id, post_id,
                                 subforum_id, forum_id),
                user_id,
            }
        }).collect();

    Ok(Comments {
        embedded: CommentList { comment_list: comments }
    })
}

pub async fn get_comment(comment_id: u64, pool: &MySqlPool) -> Result<Comment> {
    let rec = sqlx::query!(
        r#"SELECT comment, UuidFromBin(user_id) AS "user_id: String", post_id FROM comments WHERE comment_id = ?"#,
        comment_id)
        .fetch_one(pool)
        .await?;

    let subforum_id = get_subforum(rec.post_id, pool).await?;
    let forum_id = get_forum(subforum_id, pool).await?;
    let user_id = rec.user_id.unwrap();

    Ok(Comment {
        id: comment_id,
        comment_content: rec.comment,
        post_id: rec.post_id,
        links: gen_links(comment_id, &user_id, rec.post_id, subforum_id, forum_id),
        user_id
    })

}
