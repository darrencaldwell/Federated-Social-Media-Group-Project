use anyhow::Result;
use serde::{Serialize, Deserialize};
use bcrypt::hash;
use sqlx::{Row, FromRow, MySqlPool};
use crate::comments::{Comments, Comment, CommentList, gen_links as gen_comment_links, SelfLink, Link as CommentLink};
use crate::posts::{Post, PostList, Embedded as PostEmbedded, generate_post_links, parse_mariadb};
use chrono::{DateTime, Utc};
use bigdecimal::ToPrimitive;

/// Represents an entire user
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub user_id: String,
    pub description: String,
    #[serde(rename = "_links")]
    pub links: UserLinks,
}

/// Represents a local user separate from API user
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct LocalUser {
    #[serde(rename = "username")]
    pub local_username: String,
    #[serde(rename = "user_id")]
    pub local_user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(rename = "_links")]
    pub links: UserLinks,
    pub date_joined: i64,
}

/// The links sent with a [User] object
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserLinks {
    pub _self: Link,
    pub users: Link,
}

/// A list of [User] objects with links
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    #[serde(rename = "_embedded")]
    pub embedded: UsersList,
    #[serde(rename = "_links")]
    pub links: UsersLinks,
}

/// Component of [Users] containing the list of users
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersList {
    pub user_list: Vec<User>,
}

/// Component of [Users] containing the links
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersLinks {
    pub _self: Link,
}

/// A single link used by [UsersLink] and [UserLinks]
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
}

/// Represents the request to login a [User]
#[derive(Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

/// Represents the request to register a [User]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub username: String,
}

/// Represents the response from the server upon logging in
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub user_id: String,
    pub username: String,
    pub token: String,
}

/// Enumeration of all login errors
pub enum LoginError {
    InvalidHash,
}

/// Generates links to point to user and users endpoint
fn gen_links(user_id: &str) -> UserLinks {
    UserLinks {
        _self: Link {
            href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/users/{}", user_id),
        },
        users: Link {
            href: "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/users".to_string(),
        },
    }
}

/// Given a user_id and db pool queries for that user and returns it
pub async fn get_user(user_id: String, pool: &MySqlPool) -> Result<User> {
    let result = sqlx::query!("SELECT username, description FROM users WHERE user_id = (?)", &user_id)
        .fetch_one(pool)
        .await?;

    Ok(User {
        username: result.username.unwrap(),
        links: gen_links(&user_id),
        user_id,
        description: result.description,
    })
}

/// Returns a list of ALL users within our database, should probably not be used.
pub async fn get_users(pool: &MySqlPool) -> Result<Users> {
    let result = sqlx::query!(r#"SELECT user_id, username, description FROM users"#)
        .fetch_all(pool)
        .await?;

    println!("{:?}", result);
    let users: Vec<User> = result
        .into_iter()
        .map(|rec| {
            let user_id = rec.user_id;
            User {
                username: rec.username.unwrap(),
                description: rec.description,
                links: gen_links(&user_id),
                user_id: user_id.to_string(),
            }
        })
        .collect();


    Ok(Users {
        embedded: UsersList { user_list: users },
        links: UsersLinks {
            _self: Link {
                href: "https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/users".to_string(),
            },
        },
    })
}

/// Returns a list of comments for a user
pub async fn get_user_comments(user_id: String, pool: &MySqlPool) -> Result<Comments> {
    let recs = sqlx::query!(r#"SELECT comment, comments.user_id, comment_id,
        posts.subforum_id AS "subforum_id!", comments.post_id as "post_id", subforums.forum_id AS "forum_id!", username AS "username!"
        FROM comments
        LEFT JOIN users on comments.user_id = users.user_id
        LEFT JOIN posts on comments.post_id = posts.post_id
        LEFT JOIN subforums on posts.subforum_id = subforums.subforum_id
        WHERE comments.user_id = ?"#, &user_id)
        .fetch_all(pool)
        .await?;


    let comments: Vec<Comment> = recs.into_iter()
        .map(|rec| {
            Comment {
                id: rec.comment_id,
                comment_content: rec.comment,
                username: rec.username,
                post_id: rec.post_id,
                links: gen_comment_links(rec.comment_id, rec.comment_id, &rec.user_id, rec.post_id,
                                         rec.subforum_id, rec.forum_id),
                user_id: rec.user_id,
            }
        }).collect();
    Ok(Comments {
        embedded: CommentList { comment_list: comments },
        links: SelfLink {
            _self: CommentLink {
                href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/users/{}/comments", user_id)
            }
        },
    })
}

/// Returns a list of posts for a user
pub async fn get_user_posts(user_id: String, pool: &MySqlPool) -> Result<PostEmbedded> {
    let mut posts = vec![];
    let recs = sqlx::query!(
        r#"
        SELECT
            p.post_id AS "post_id!", post_title AS "post_title!", p.user_id AS "user_id!",
            post_contents AS "post_contents!", p.subforum_id AS "subforum_id!", forum_id AS "forum_id!",
            sum(case when pv.is_upvote = 0 then 1 else 0 end) AS "downvotes!",
            sum(case when pv.is_upvote = 1 then 1 else 0 end) AS "upvotes!",
            JSON_OBJECT("postsVotes", JSON_ARRAYAGG(
                JSON_OBJECT("isUpvote", (CASE WHEN is_upvote = 1 then true WHEN is_upvote = 0 THEN false END), "user",
                    CONCAT(i.implementation_url, '/api/users/', pv.user_id)))
            ) AS "user_votes"
        FROM posts p
        INNER JOIN subforums s on p.subforum_id = s.subforum_id
        LEFT JOIN posts_votes pv ON
            p.post_id = pv.post_id
        LEFT JOIN implementations i ON
            pv.implementation_id = i.implementation_id
        WHERE p.user_id = ?
        ORDER BY created_time
        "#,
        &user_id
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
            downvotes: rec.downvotes.to_u64().unwrap(),
            upvotes: rec.upvotes.to_u64().unwrap(),
            user_votes,
            links: generate_post_links(
                rec.post_id,
                rec.subforum_id,
                rec.forum_id,
                &user_id,
            ),
            user_id: user_id.to_string(),
        });
    }
    let post_list = PostList { post_list: posts };
    let embedded = PostEmbedded {
        _embedded: post_list,
    };
    Ok(embedded)
}


/// Get account details
pub async fn get_account(user_id: String, pool: &MySqlPool) -> Result<LocalUser> {
    let rec = sqlx::query!(
        r#"SELECT username, first_name, last_name, user_id, email, date_joined FROM users WHERE user_id = (?) and implementation_id = 1"#,
        user_id) //get comments
        .fetch_one(pool)
        .await?;

    Ok(LocalUser {
        links: gen_links(&user_id),
        local_username: rec.username.unwrap(),
        local_user_id: user_id,
        first_name: rec.first_name.unwrap(),
        last_name: rec.last_name.unwrap(),
        email: rec.email.unwrap(),
        date_joined: rec.date_joined.timestamp(),
    })
}

/// Registers a [user] into the database and returns a [user] object
pub async fn register(username: String, password: String, first_name: String, last_name: String, email: String, pool: &MySqlPool) -> Result<LocalUser> {
    let tx = pool.begin().await?;
    let password_hash: String = hash(password, 10)?;


    let rec = sqlx::query!(
        r#"insert into users (username, password_hash, user_id, implementation_id, first_name, last_name, email) values(?, ?, UUID(), ?, ?, ?, ?) RETURNING user_id, date_joined"#,
        username,
        password_hash,
        1,
        first_name,
        last_name,
        email
    )
        .fetch_one(pool)
        .await?;

    tx.commit().await?;
    let user_id: String = rec.get(0);
    let date_joined: DateTime<Utc> = rec.get(1);
    let date_joined_ts = date_joined.timestamp();
    // let date_joined = rec.get::<DateTime<Utc>>(1);

    let local_user = LocalUser {
        links: gen_links(&user_id),
        local_username: username,
        local_user_id: user_id,

        first_name,
        last_name,
        email,
        date_joined: date_joined_ts,
    };

    Ok(local_user)
}


/// Used for verifying a login attempt, checks that the credentials match
pub async fn verify(
    username: &str,
    password: &str,
    pool: &MySqlPool,
) -> Result<String, LoginError> {
    let rec = sqlx::query!(
        r#"SELECT password_hash, user_id FROM users WHERE username = ?"#,
        username
    )
        // Uniqueness guaranteed by database
        .fetch_one(pool)
        .await;

    let rec_result = match rec {
        Ok(rec) => rec,
        Err(e) => {
            println!("{:?}", e);
            return Err(LoginError::InvalidHash);
        }
    };

    let password_hash = match rec_result.password_hash {
        Some(password_hash) => password_hash,
        None => {
            println!("Trying to login foreign user.");
            return Err(LoginError::InvalidHash);
        }
    };

    match bcrypt::verify(password, &password_hash) {
        Ok(boolean) => if !boolean { // if password hash doesn't match
            return Err(LoginError::InvalidHash);
        },
        Err(_) => return Err(LoginError::InvalidHash),
    };

    Ok(rec_result.user_id)
}
