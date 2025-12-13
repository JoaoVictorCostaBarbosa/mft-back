use thiserror::Error;

#[derive(Debug, Error)]
pub enum PermissionError {
    #[error("invalid credentials")]
    Unauthorized,
    #[error("user don't have permissions")]
    Forbidden,
}