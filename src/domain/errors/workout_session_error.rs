use crate::domain::value_objects::NameError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkoutSessionError {
    #[error("workout session not found")]
    NotFound,

    #[error("workout session already in progress")]
    AlreadyInProgress,

    #[error("workout session already finished")]
    AlreadyFinished,

    #[error("workout session already cancelled")]
    AlreadyCancelled,

    #[error("workout session must be in progress")]
    NotInProgress,

    #[error("invalid exercise order")]
    InvalidExerciseOrder,

    #[error("finished_at cannot be before started_at")]
    InvalidFinishedAt,

    #[error("invalid reps")]
    InvalidReps,

    #[error("invalid weight")]
    InvalidWeight,

    #[error("invalid name error: {0}")]
    InvalidName(#[from] NameError),
}
