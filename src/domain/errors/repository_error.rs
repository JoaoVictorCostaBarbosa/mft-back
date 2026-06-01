use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("entity not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("database error: {0}")]
    DbError(String),

    #[error("unexpected error: {0}")]
    Unexpected(String),
}
