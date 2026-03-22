use crate::domain::{
    errors::exercise_log_error::ExerciseLogError, value_objects::name_vo::NameError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkoutLogError {
    #[error("workout log not found")]
    NotFound,

    #[error("without permission to access the resource")]
    Forbidden,

    #[error("exercise log error: {0}")]
    ExerciseLog(#[from] ExerciseLogError),

    #[error("invalid name error: {0}")]
    InvalidName(#[from] NameError),
}
