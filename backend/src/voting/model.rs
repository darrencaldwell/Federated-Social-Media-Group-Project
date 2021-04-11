use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Done};
use super::super::request_errors::RequestError;

/*
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserVotes {
    #[serde(rename = "_userVotes")]
    pub user_votes: Vec<UserVote>,
}
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserVote {
    pub is_upvote: Option<bool>,
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteRequest {
    pub is_upvote: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct DbUserVotes {
    #[serde(rename = "_userVotes")]
    user_votes: Vec<UserVote>,
}

/// Deals with the bug in MariaDB where sometimes it incorrectly formats JSON
pub fn parse_mariadb(json_string: String) -> Vec<UserVote> {
    // MariaDB has a bug where it forgets to add the brackets when there are < 7 votes in the
    // database, so we have to manually correct that
    let db_user_votes: DbUserVotes = match serde_json::from_str(&json_string) {
        Ok(user_votes) => {
            user_votes
        },
        Err(_) => {
            log::info!("old_string: {:?}",&json_string);
            if json_string.starts_with("{\"_userVotes\": \"[") {
                // replace \"[ , ]\"
                let tmp = json_string.replace("\"[", "[");
                let tmp = tmp.replace("]\"", "]");
                let tmp = tmp.replace("\\","");
                serde_json::from_str::<DbUserVotes>(&tmp).unwrap()
            } else {
                let tmp = json_string.replace("{\"_userVotes\": \"{", "{\"_userVotes\": [{");
                let mut tmp2 = tmp.replace("\\","");
                tmp2.replace_range(tmp2.len()-3..tmp2.len(), "}]}");
                serde_json::from_str(&tmp2).unwrap()
            }
        },
    };
    // a small hack to remove any null results, would take much more code to write a custom
    // vec deserializer / serializer and it's only like N complexity
    let mut user_votes = db_user_votes.user_votes;
    user_votes.retain(|x| x.user.is_some() && x.is_upvote.is_some());
    user_votes
}
/// Modifies an existing post
pub async fn put_post_vote(post_id: u64, user_id: String, implementation_id: u64, vote: VoteRequest, pool: &MySqlPool) -> Result<(), RequestError> {
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
        if number_modified == 0 {
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

    if number_modified == 0 {return Err(RequestError::NotFound(format!("post_id: {} not found to upvote", post_id)))} 
    Ok(())
}

/// Modifies an existing comments
pub async fn put_comment_vote(comment_id: u64, user_id: String, implementation_id: u64, vote: VoteRequest, pool: &MySqlPool) -> Result<(), RequestError> {
    let is_upvote = vote.is_upvote;

    // delete if theres no vote
    if is_upvote.is_none() {
        let number_modified = sqlx::query!(
            r#"
            DELETE FROM comments_votes
            WHERE comment_id = ? AND user_id = ? AND implementation_id = ?
            "#,
            comment_id,
            user_id,
            implementation_id
            )
            .execute(pool)
            .await?
            .rows_affected();
        if number_modified == 0 {
            return Err(RequestError::NotFound(format!("post_id: {} not found for user: {}#{}", comment_id, user_id, implementation_id)))
        }
        return Ok(())
    }

    let number_modified = sqlx::query!(
        r#"
        INSERT INTO comments_votes
        (comment_id, user_id, implementation_id, is_upvote)
        VALUES (?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE is_upvote = ?
        "#,
        comment_id,
        user_id,
        implementation_id,
        is_upvote.unwrap(), is_upvote.unwrap()
    )
    .execute(pool)
    .await?
    .rows_affected();

    if number_modified == 0 {return Err(RequestError::NotFound(format!("comment_id: {} not found to upvote", comment_id)))} 
    Ok(())
}
