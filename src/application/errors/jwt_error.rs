use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("expired token")]
    ExpiredToken,

    #[error("invalid token")]
    InvalidToken,

    #[error("invalid signature")]
    InvalidSignature,

    #[error("missing claim")]
    MissingClaim,

    #[error("internal jwt error: {0}")]
    Internal(String),
}
