use anyhow::Result;
use serde::Serialize;

use sqlx::{FromRow, MySqlPool};
use bcrypt::hash;

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

fn gen_links(user_id: u64) -> UserLinks {
    UserLinks {
        _self: Link { href: format!("<url>/api/users/{}", user_id) },
        users: Link { href: format!("<url>/api/users") },
    }
}

pub async fn get_users(pool: &MySqlPool) -> Result<Users> {
    let result = sqlx::query!("SELECT user_id, username FROM users")
        .fetch_all(pool)
        .await?;

    let users: Vec<User> = result.into_iter()
        .map(|rec| User {
            username: rec.username,
            user_id: rec.user_id,
            links: gen_links(rec.user_id)
        })
        .collect();

    Ok(Users {
        embedded: UsersList { user_list: users },
        links: UsersLinks { _self: Link { href: format!("<url>/api/users") } }
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

pub async fn verify(username: &String, password: &String, pool: &MySqlPool) -> bool {
    let hash = sqlx::query!(
        "SELECT password_hash FROM users WHERE username = ?",
        username
    )
    // Uniqueness guaranteed by database
    .fetch_one(pool)
    .await;

    let password_hash = match hash {
        Ok(hash) => hash.password_hash,
        Err(_) => return false,
    };

    match bcrypt::verify(password, &password_hash) {
        Ok(o) => o,
        Err(_) => false,
    }
}
