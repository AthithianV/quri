use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuriError {
    #[error("storage error: {0}")]
    Storage(anyhow::Error),

    #[error("connection error: {0}")]
    Connection(anyhow::Error),

    #[error("query error: {0}")]
    Query(anyhow::Error),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("extension error: {0}")]
    Extension(String),

    #[error("internal error: {0}")]
    Internal(anyhow::Error),
}

pub type QuriResult<T> = Result<T, QuriError>;
