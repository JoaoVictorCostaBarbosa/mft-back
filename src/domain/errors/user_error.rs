use crate::domain::value_objects::EmailError;
use crate::domain::value_objects::NameError;
use crate::domain::value_objects::PasswordError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("invalid name: {0}")]
    NameInvalid(#[from] NameError),

    #[error("invalid email: {0}")]
    EmailInvalid(#[from] EmailError),

    #[error("invalid password {0}")]
    PasswordInvalid(#[from] PasswordError),
}
