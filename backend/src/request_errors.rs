use thiserror::Error;

    #[derive(Error, Debug)]
pub enum RequestError {
    #[error("Item not found: {0}")]
    NotFound(String),
    #[error("sqlx error: `{0}`")]
    SqlxError(#[from] sqlx::Error)
}
