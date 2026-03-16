use crate::domain::errors::set_log_error::SetLogError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExerciseLogError {
    #[error("exercise log cannot be empty.")]
    EmptyExercise,
    
    #[error("exercise log not found")]
    NotFound,

    #[error("set log error: {0}")]
    SetLog(#[from] SetLogError),
}
