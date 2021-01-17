use anyhow::Result;
use serde::Serialize;

use bcrypt::hash;
use sqlx::{FromRow, MySqlPool};

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub user_id: String,
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

fn gen_links(user_id: String) -> UserLinks {
    UserLinks {
        _self: Link {
            href: format!("<url>/api/users/{}", user_id),
        },
        users: Link {
            href: format!("<url>/api/users"),
        },
    }
}

pub async fn get_user(user_id: String, pool: &MySqlPool) -> Result<User> {
    let username = sqlx::query!("SELECT username FROM users WHERE user_id = (UuidToBin(?))", user_id.clone())
        .fetch_one(pool)
        .await?
        .username;

    Ok(User {
        username,
        links: gen_links(user_id.clone()),
        user_id,
    })
}

pub async fn get_users(pool: &MySqlPool) -> Result<Users> {
    let result = sqlx::query!("SELECT UuidFromBin(user_id) AS user_id, username FROM users")
        .fetch_all(pool)
        .await?;

    // TODO: check this, feels like a lot of shenanignas. - darren
    let users: Vec<User> = result
        .into_iter()
        .map(|rec| User {
            username: rec.username,
            links: gen_links(rec.user_id.clone().unwrap().as_str().unwrap().to_string()),
            user_id: rec.user_id.unwrap().as_str().unwrap().to_string()
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

    // TODO: if fixit update mariaDB we can just return the ID after this first query and no longer
    // need the second one. - Darren
    sqlx::query!(
        //"insert into users (username, password_hash, user_id) values(?, ?, UuidToBin(UUID())) RETURNING user_id",
        "insert into users (username, password_hash, user_id) values(?, ?, UuidToBin(UUID()))",
        username,
        password_hash
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;

    // Note need to grab user_id as STRING else we all cry
    let user_id = sqlx::query!(
        r#"SELECT UuidFromBin(user_id) AS "user_id: String" FROM users WHERE username = ?"#, username
    )
    .fetch_one(pool)
    .await?;

    let uuid: String = user_id.user_id.unwrap().as_str().to_string();
    println!("{:?}", uuid.clone());
    let new_user = User {
        username,
        links: gen_links(uuid.clone()),
        user_id: uuid
    };

    Ok(new_user)
}

pub async fn verify(
    username: &String,
    password: &String,
    pool: &MySqlPool,
) -> Result<String, LoginError> {
    let rec = sqlx::query!(
        r#"SELECT password_hash, UuidFromBin(user_id) AS "user_id: String" FROM users WHERE username = ?"#,
        username
    )
    // Uniqueness guaranteed by database
    .fetch_one(pool)
    .await;

    let rec_result = match rec {
        Ok(rec) => rec,
        Err(e) => {
            println!("{:?}",e);
            return Err(LoginError::InvalidHash)
        },
    };

    match bcrypt::verify(password, &rec_result.password_hash) {
        Ok(boolean) => if !boolean { // if password hash doesn't match
            return Err(LoginError::InvalidHash)
        },
        Err(_) => return Err(LoginError::InvalidHash),
    };

    Ok(rec_result.user_id.unwrap().as_str().to_string())
}
