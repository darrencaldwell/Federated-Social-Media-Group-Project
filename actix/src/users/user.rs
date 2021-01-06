use anyhow::Result;
use serde::Serialize;

use bcrypt::hash;
use sqlx::{FromRow, MySqlPool};

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub user_id: u64,
    #[serde(rename = "_links")]
    pub links: UserLinks,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserLinks {
    pub _self: Link,
    pub users: Link,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    #[serde(rename = "_embedded")]
    pub embedded: UsersList,
    #[serde(rename = "_links")]
    pub links: UsersLinks,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersList {
    pub user_list: Vec<User>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersLinks {
    pub _self: Link,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
}

pub enum LoginError {
    InvalidHash,
}

fn gen_links(user_id: u64) -> UserLinks {
    UserLinks {
        _self: Link {
            href: format!("<url>/api/users/{}", user_id),
        },
        users: Link {
            href: format!("<url>/api/users"),
        },
    }
}

pub async fn get_user(user_id: u64, pool: &MySqlPool) -> Result<User> {
    let username = sqlx::query!("SELECT username FROM users WHERE user_id = ?", user_id)
        .fetch_one(pool)
        .await?
        .username;

    Ok(User {
        username,
        user_id,
        links: gen_links(user_id),
    })
}

pub async fn get_users(pool: &MySqlPool) -> Result<Users> {
    let result = sqlx::query!("SELECT user_id, username FROM users")
        .fetch_all(pool)
        .await?;

    let users: Vec<User> = result
        .into_iter()
        .map(|rec| User {
            username: rec.username,
            user_id: rec.user_id,
            links: gen_links(rec.user_id),
        })
        .collect();

    Ok(Users {
        embedded: UsersList { user_list: users },
        links: UsersLinks {
            _self: Link {
                href: format!("<url>/api/users"),
            },
        },
    })
}

pub async fn register(username: String, password: String, pool: &MySqlPool) -> Result<User> {
    let mut tx = pool.begin().await?;
    let password_hash: String = hash(password, 10)?;

    let user_id = sqlx::query!(
        "insert into users (username, password_hash) values(?, ?)",
        username,
        password_hash
    )
    .execute(&mut tx)
    .await?
    .last_insert_id();

    tx.commit().await?;

    let new_user = User {
        username,
        user_id,
        links: gen_links(user_id),
    };

    Ok(new_user)
}

pub async fn verify(
    username: &String,
    password: &String,
    pool: &MySqlPool,
) -> Result<u64, LoginError> {
    let rec = sqlx::query!(
        "SELECT password_hash, user_id FROM users WHERE username = ?",
        username
    )
    // Uniqueness guaranteed by database
    .fetch_one(pool)
    .await;

    let rec_result = match rec {
        Ok(rec) => rec,
        Err(_) => return Err(LoginError::InvalidHash),
    };

    match bcrypt::verify(password, &rec_result.password_hash) {
        Ok(boolean) => if !boolean { // if password hash doesn't match
            return Err(LoginError::InvalidHash)
        },
        Err(_) => return Err(LoginError::InvalidHash),
    };

    Ok(rec_result.user_id)
}
