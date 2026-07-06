use crate::domain::value_objects::NameError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExerciseError {
    #[error("invalid name: {0}")]
    NameInvalid(#[from] NameError),

    #[error("invalid filds criteria")]
    InvalidFieldsCriteria,

    #[error("exercise not found")]
    NotFound,
}
