use crate::domain::{
    entities::{user::User, workout_session::WorkoutSessionHistoryItem},
    errors::domain_error::DomainError,
    repositories::workout_session_repository::WorkoutSessionRepository,
};
use std::sync::Arc;

pub struct ReadWorkoutSessionHistory {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
}

impl ReadWorkoutSessionHistory {
    pub fn new(workout_session_repo: Arc<dyn WorkoutSessionRepository>) -> Self {
        Self {
            workout_session_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
    ) -> Result<Vec<WorkoutSessionHistoryItem>, DomainError> {
        self.workout_session_repo.history(current_user.id).await
    }
}
