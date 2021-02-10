use anyhow::Result;
use serde::{Serialize, Deserialize};
use bcrypt::hash;
use sqlx::{Row, FromRow, MySqlPool};

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
fn gen_links(user_id: &String) -> UserLinks {
    UserLinks {
        _self: Link {
            href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/users/{}", user_id),
        },
        users: Link {
            href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/users"),
        },
    }
}

/// Given a user_id and db pool queries for that user and returns it
pub async fn get_user(user_id: String, pool: &MySqlPool) -> Result<User> {
    let result = sqlx::query!("SELECT username, description FROM users WHERE user_id = (UuidToBin(?))", &user_id)
        .fetch_one(pool)
        .await?;

    Ok(User {
        username: result.username,
        links: gen_links(&user_id),
        user_id,
        description: result.description,
    })
}

/// Returns a list of ALL users within our database, should probably not be used.
pub async fn get_users(pool: &MySqlPool) -> Result<Users> {
    let result = sqlx::query!(r#"SELECT UuidFromBin(user_id) AS "user_id: String", username, description FROM users"#)
        .fetch_all(pool)
        .await?;

    println!("{:?}",result);
    let users: Vec<User> = result
        .into_iter()
        .map(|rec| {
            let user_id = rec.user_id.unwrap();
            User {
                username: rec.username,
                description: rec.description,
                links: gen_links(&user_id),
                user_id,
            }
        })
        .collect();


    Ok(Users {
        embedded: UsersList { user_list: users },
        links: UsersLinks {
            _self: Link {
                href: format!("https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/users"),
            },
        },
    })
}

/// Registers a [user] into the database and returns a [user] object
pub async fn register(username: String, password: String, pool: &MySqlPool) -> Result<User> {
    let tx = pool.begin().await?;
    let password_hash: String = hash(password, 10)?;
    let default_desc = String::from("");

    let user_id: String = sqlx::query!(
        r#"insert into users (username, password_hash, user_id, server, description) values(?, ?, UuidToBin(UUID()), ?, ?) RETURNING UuidFromBin(user_id) AS user_id"#,
        username,
        password_hash,
        "local",
        default_desc
    )
    .fetch_one(pool)
    .await?
    // Due to either mariaDB or sqlx, it seems to think the return result is a binary and not a
    // string, since we know it will be a valid string we can use this get_unchecked to get the
    // UUID string.
    .get_unchecked(0);

    tx.commit().await?;

    let new_user = User {
        username,
        description: default_desc,
        links: gen_links(&user_id),
        user_id: user_id
    };

    Ok(new_user)
}

/// Used for verifying a login attempt, checks that the credentials match
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

    let password_hash = match rec_result.password_hash {
        Some(password_hash) => password_hash,
        None => {
            println!("Trying to login foreign user.");
            return Err(LoginError::InvalidHash)
        },
    };

    match bcrypt::verify(password, &password_hash) {
        Ok(boolean) => if !boolean { // if password hash doesn't match
            return Err(LoginError::InvalidHash)
        },
        Err(_) => return Err(LoginError::InvalidHash),
    };

    Ok(rec_result.user_id.unwrap())
}
