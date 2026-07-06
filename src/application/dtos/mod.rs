pub mod auth;
pub mod exercise;
pub mod measurements;
pub mod user;
mod workout_plan;
pub mod workout_session;
pub mod workout_template;

pub use workout_plan::{
    AddRoutineItemInput, UpdateRoutineItemInput, WorkoutPlanRequest, WorkoutPlanUpdateRequest,
};
