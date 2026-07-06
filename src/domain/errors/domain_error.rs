use crate::domain::errors::ExerciseError;
use crate::domain::errors::MeasurementError;
use crate::domain::errors::PermissionError;
use crate::domain::errors::RepositoryError;
use crate::domain::errors::UserError;
use crate::domain::errors::WorkoutPlanError;
use crate::domain::errors::WorkoutSessionError;
use crate::domain::errors::WorkoutTemplateError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("user error: {0}")]
    User(#[from] UserError),

    #[error("permission error: {0}")]
    Permission(#[from] PermissionError),

    #[error("measurement error: {0}")]
    Measurement(#[from] MeasurementError),

    #[error("exercise error: {0}")]
    Exercise(#[from] ExerciseError),

    #[error("workout plan error: {0}")]
    WorkoutPlan(#[from] WorkoutPlanError),

    #[error("workout template error {0}")]
    WorkoutTemplate(#[from] WorkoutTemplateError),

    #[error("workout session error {0}")]
    WorkoutSession(#[from] WorkoutSessionError),
}
