use crate::domain::{
    errors::workout_template_error::WorkoutTemplateError, value_objects::name_vo::NameError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkoutPlanError {
    #[error("invalid name error: {0}")]
    NameInvalid(#[from] NameError),

    #[error("workout already added")]
    AlreadyAdded,

    #[error("forbidden error")]
    Forbidden,

    #[error("workout template error: {0}")]
    WorkoutTemplate(#[from] WorkoutTemplateError),
}
