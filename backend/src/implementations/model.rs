use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, Done};
use super::super::request_errors::RequestError;

/// Represents a request to POST (or PUT to replace) an implementation
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationRequest {
    pub url: String,
    pub name: String,
}

/// Represents an implementation in our database
#[derive(Serialize, FromRow)]
pub struct Implementation {
    pub id: u64,
    pub url: String,
    pub name: String,
}
/// The root of the JSON object, contains all implementations
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Embedded {
    #[serde(rename = "_embedded")]
    _embedded: ImplementationList,
}

/// Implementation List
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationList {
    implementation_list: Vec<Implementation>,
}

///  Get a single implementation by its id
pub async fn get_one(id: u64, pool: &MySqlPool) -> Result<Implementation> {
    let rec = sqlx::query!(
        r#"
        SELECT implementation_url, implementation_name FROM implementations
        WHERE implementation_id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    let implementation = Implementation {
        id,
        name: rec.implementation_name,
        url: rec.implementation_url,
    };
    Ok(implementation)
}
/// Get all implementations in the database
pub async fn get_all(pool: &MySqlPool) -> Result<Embedded, RequestError> {
    let mut implementations = vec![];
    let recs = sqlx::query!(
        r#"
        SELECT implementation_id, implementation_name, implementation_url
        FROM implementations
        ORDER BY implementation_id
        "#,
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        implementations.push(Implementation {
            id: rec.implementation_id,
            url: rec.implementation_url,
            name: rec.implementation_name,
        })
    }
    let implementation_list = ImplementationList { implementation_list: implementations };
    let embedded = Embedded {
        _embedded: implementation_list,
    };
    Ok(embedded)
}
/// Add a new supported implementation to the database
pub async fn post(implementation: ImplementationRequest, pool: &MySqlPool) -> Result<Implementation, RequestError> {
    // pool is used for a transaction, ie a rollbackable operation
    let mut tx = pool.begin().await?;

    let id = sqlx::query!(
        r#"
        insert into implementations (implementation_url, implementation_name)
        values( ?, ?)
        "#,
        implementation.url,
        implementation.name
    )
    .execute(&mut tx)
    .await?
    .last_insert_id();

    // commits transaction, ie "cements" it
    tx.commit().await?;

    // return the implementation as if it was retrieved by a GET
    let new_implementation = Implementation {
        id,
        url: implementation.url,
        name: implementation.name,
    };
    Ok(new_implementation)
}
/// Modifies an existing implementation in the database by id
pub async fn put(implementation_id: u64, implementation: ImplementationRequest, pool: &MySqlPool) -> Result<(), RequestError> {
    let number_modified = sqlx::query!(
        r#"
        UPDATE implementations
        SET implementation_name = ?, implementation_url = ?
        WHERE implementation_id = ?
        "#,
        implementation.name,
        implementation.url,
        implementation_id,
    )
    .execute(pool)
    .await?
    .rows_affected();

    if number_modified == 0 {
        Err(RequestError::NotFound(format!("implementation_id: {} not found", implementation_id)))
    } else {
        Ok(())
    }
}
/// Deletes an implementation from the database by id
pub async fn delete(implementation_id: u64, pool: &MySqlPool) -> Result<(), RequestError> {
    let number_modified = sqlx::query!(
        r#"
        DELETE from implementations WHERE implementation_id = ?
        "#,
        implementation_id,
    )
    .execute(pool)
    .await?
    .rows_affected();

    if number_modified == 0 {
        Err(RequestError::NotFound(format!("implementation_id: {} not found", implementation_id)))
    } else {
        Ok(())
    }
}
