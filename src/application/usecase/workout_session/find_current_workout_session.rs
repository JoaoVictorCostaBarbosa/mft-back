use crate::domain::{
    entities::{user::User, workout_session::CurrentWorkoutSession},
    errors::domain_error::DomainError,
    repositories::workout_session_repository::WorkoutSessionRepository,
};
use std::sync::Arc;

pub struct FindCurrentWorkoutSession {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
}

impl FindCurrentWorkoutSession {
    pub fn new(workout_session_repo: Arc<dyn WorkoutSessionRepository>) -> Self {
        Self {
            workout_session_repo,
        }
    }

    pub async fn execute(&self, current_user: User) -> Result<CurrentWorkoutSession, DomainError> {
        self.workout_session_repo
            .find_current(current_user.id)
            .await
    }
}
