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

    #[error("workout session already in progress")]
    AlreadyInProgress,

    #[error("workout session already finished")]
    AlreadyFinished,

    #[error("finished_at cannot be before started_at")]
    InvalidFinishedAt,

    #[error("invalid reps")]
    InvalidReps,

    #[error("invalid weight")]
    InvalidWeight,

    #[error("exercise log error: {0}")]
    ExerciseLog(#[from] ExerciseLogError),

    #[error("invalid name error: {0}")]
    InvalidName(#[from] NameError),
}
