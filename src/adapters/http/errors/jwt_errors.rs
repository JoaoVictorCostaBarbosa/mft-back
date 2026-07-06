use crate::application::errors::JwtError;
use jsonwebtoken::errors::{Error, ErrorKind};

impl From<Error> for JwtError {
    fn from(err: Error) -> Self {
        match err.kind() {
            ErrorKind::ExpiredSignature => JwtError::ExpiredToken,
            ErrorKind::InvalidToken => JwtError::InvalidToken,
            ErrorKind::InvalidSignature => JwtError::InvalidSignature,
            ErrorKind::MissingRequiredClaim(_) => JwtError::MissingClaim,
            other => JwtError::Internal(format!("jwt error: {:?}", other)),
        }
    }
}
