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

    #[error("day of week already scheduled")]
    DayAlreadyScheduled,

    #[error("position already scheduled")]
    PositionAlreadyScheduled,

    #[error("workout template is required for workout routine items")]
    WorkoutTemplateRequired,

    #[error("rest routine items cannot have workout templates")]
    RestCannotHaveWorkoutTemplate,

    #[error("weekly routines do not use position")]
    WeeklyRoutineDoesNotUsePosition,

    #[error("weekly routines require day of week")]
    WeeklyRoutineRequiresDayOfWeek,

    #[error("sequential routines do not use day of week")]
    SequentialRoutineDoesNotUseDayOfWeek,

    #[error("sequential routines require position")]
    SequentialRoutineRequiresPosition,

    #[error("forbidden error")]
    Forbidden,

    #[error("workout template error: {0}")]
    WorkoutTemplate(#[from] WorkoutTemplateError),
}
