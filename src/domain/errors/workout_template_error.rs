use crate::domain::errors::ExerciseError;
use crate::domain::value_objects::NameError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkoutTemplateError {
    #[error("invalid name error: {0}")]
    NameInvalid(#[from] NameError),

    #[error("exercise already added")]
    AlreadyAdded,

    #[error("exercise error: {0}")]
    Exercise(#[from] ExerciseError),

    #[error("workout template not found")]
    NotFound,
}
