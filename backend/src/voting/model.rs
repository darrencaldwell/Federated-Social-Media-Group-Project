use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, Done};
use serde::ser::{Serializer, SerializeStruct};
use super::super::request_errors::RequestError;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteRequest {
    pub is_upvote: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsVotes {
    pub posts_votes: Vec<PostVotes>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostVotes {
    pub post_id: u64,
    pub total_votes: u64,
    pub upvote_count: u64,
    // if user has reacted then these will exist
    pub user_id: Option<String>,
    pub is_upvote: Option<bool>,
}

/// Modifies an existing post
pub async fn put_vote(post_id: u64, user_id: String, implementation_id: u64, vote: VoteRequest, pool: &MySqlPool) -> Result<(), RequestError> {
    let is_upvote = vote.is_upvote;

    // delete if theres no vote
    if is_upvote.is_none() {
        let number_modified = sqlx::query!(
            r#"
            DELETE FROM posts_votes
            WHERE post_id = ? AND user_id = ? AND implementation_id = ?
            "#,
            post_id,
            user_id,
            implementation_id
            )
            .execute(pool)
            .await?
            .rows_affected();
        if number_modified != 1 {
            return Err(RequestError::NotFound(format!("post_id: {} not found for user: {}#{}", post_id, user_id, implementation_id)))
        }
        return Ok(())
    }

    let number_modified = sqlx::query!(
        r#"
        INSERT INTO posts_votes
        (post_id, user_id, implementation_id, is_upvote)
        VALUES (?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE is_upvote = ?
        "#,
        post_id,
        user_id,
        implementation_id,
        is_upvote.unwrap(), is_upvote.unwrap()
    )
    .execute(pool)
    .await?
    .rows_affected();

    if number_modified != 1 {return Err(RequestError::NotFound(format!("post_id: {} not found to upvote", post_id)))} 
    Ok(())
}

/// Get all the votes for all posts in a subforum, including information about whether the user who
/// requested it has voted on any of them
pub async fn get_posts_votes(subforum_id: u64, user_id: String, implementation_id: u64,pool: &MySqlPool) -> Result<PostsVotes> {
    let mut posts_votes = vec![];
    let recs = sqlx::query!(
        r#"
		SELECT 
            pv1.post_id,
            count(pv1.post_id) AS "total_votes: u64",
            sum(case when pv1.is_upvote = 1 then 1 else 0 end) AS "upvote_count: u64",
            pv2.user_id,
            pv2.is_upvote
        FROM posts_votes pv1
        LEFT JOIN posts p2 ON pv1.post_id = p2.post_id
        LEFT JOIN subforums s2 ON p2.subforum_id = s2.subforum_id
        LEFT JOIN posts_votes pv2 ON 
            pv1.post_id = pv2.post_id AND
            pv1.is_upvote = pv2.is_upvote AND
            pv1.implementation_id = ? AND
            pv1.user_id = ?
        WHERE p2.subforum_id = ?
        GROUP BY post_id
        "#,
        implementation_id,
        user_id,
        subforum_id
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        let is_upvote;
        if rec.is_upvote.is_some() {
            if rec.is_upvote.unwrap() > 0 {
                is_upvote = Some(true);
            } else {is_upvote = Some(false);}
        } else {is_upvote = None;}
        posts_votes.push(PostVotes {
            post_id: rec.post_id,
            total_votes: rec.total_votes,
            upvote_count: rec.upvote_count.unwrap(),
            user_id: rec.user_id,
            is_upvote,
        });
    }
    let post_list = PostsVotes { posts_votes };
    Ok(post_list)
}
