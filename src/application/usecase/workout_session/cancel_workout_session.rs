use crate::domain::{
    entities::user::User,
    errors::{domain_error::DomainError, workout_log_error::WorkoutLogError},
    repositories::workout_session_repository::WorkoutSessionRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct CancelWorkoutSession {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
}

impl CancelWorkoutSession {
    pub fn new(workout_session_repo: Arc<dyn WorkoutSessionRepository>) -> Self {
        Self {
            workout_session_repo,
        }
    }

    pub async fn execute(&self, current_user: User, session_id: Uuid) -> Result<(), DomainError> {
        let session = self.workout_session_repo.find_by_id(session_id).await?;

        if session.user_id != current_user.id {
            return Err(WorkoutLogError::Forbidden.into());
        }

        session.assert_in_progress()?;
        self.workout_session_repo.cancel(session_id).await
    }
}
