use crate::domain::{errors::exercise_error::ExerciseError, value_objects::name_vo::NameError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkoutTemplateError {
    #[error("invalid name error: {0}")]
    NameInvalid(#[from] NameError),

    #[error("exercise already added")]
    AlreadyAdded,

    #[error("exercise error: {0}")]
    Exercise(#[from] ExerciseError),

    #[error("forbidden error")]
    Forbidden,

    #[error("workout template not found")]
    NotFound,
}
