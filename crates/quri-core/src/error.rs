pub enum QuriError {
    Storage(anyhow::Error),
    Connection(anyhow::Error),
    Query(anyhow::Error),
    Validation(String),
    NotFound(String),
    Extension(String),
    Internal(anyhow::Error),
}

pub type QuriResult<T> = Result<T, QuriError>;
