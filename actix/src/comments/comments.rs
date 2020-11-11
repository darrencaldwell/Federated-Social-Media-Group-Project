use sqlx::MySqlPool;
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: u64,
    pub comment_content: String,
    pub user_id: u64,
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
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
}

fn gen_links(comment_id: u64, user_id: u64, post_id: u64, subforum_id: u64, forum_id: u64) -> Links {
    Links {
        _self: Link { href: format!("<url>/api/comments/{}", comment_id) },
        post: Link { href: format!("<url>/api/posts/{}", post_id) },
        subforum: Link { href: format!("<url>/api/subforums/{}", subforum_id) },
        forum: Link { href: format!("<url>/api/forums/{}", forum_id) },
        user: Link { href: format!("<url>/api/user/{}", user_id) },
    }
}

pub async fn get_comment(comment_id: u64, pool: &MySqlPool) -> Result<Comment> {
    let rec = sqlx::query!(
        "SELECT comment, user_id, post_id FROM comments WHERE comment_id = ?",
        comment_id)
        .fetch_one(pool)
        .await?;

    let subforum_id = sqlx::query!(
        "SELECT subforum_id FROM posts WHERE post_id = ?",
        rec.post_id)
        .fetch_one(pool)
        .await?
        .subforum_id;

    let forum_id = sqlx::query!(
        " SELECT forum_id FROM subforums WHERE subforum_id = ?",
        subforum_id)
        .fetch_one(pool)
        .await?
        .forum_id;

    Ok(Comment {
        id: comment_id,
        comment_content: rec.comment,
        user_id: rec.user_id,
        post_id: rec.post_id,
        links: gen_links(comment_id, rec.user_id, rec.post_id, subforum_id, forum_id),
    })

}
