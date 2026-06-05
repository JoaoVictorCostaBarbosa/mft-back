use crate::domain::{
    entities::{user::User, workout_session::FinishedWorkoutSession},
    errors::{domain_error::DomainError, workout_log_error::WorkoutLogError},
    repositories::workout_session_repository::WorkoutSessionRepository,
};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub struct FinishWorkoutSession {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
}

impl FinishWorkoutSession {
    pub fn new(workout_session_repo: Arc<dyn WorkoutSessionRepository>) -> Self {
        Self {
            workout_session_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        session_id: Uuid,
        finished_at: Option<DateTime<Utc>>,
    ) -> Result<FinishedWorkoutSession, DomainError> {
        let session = self.workout_session_repo.find_by_id(session_id).await?;

        if session.user_id != current_user.id {
            return Err(WorkoutLogError::Forbidden.into());
        }

        let finished = session.finish(finished_at)?;
        self.workout_session_repo.finish(&finished).await?;

        Ok(finished)
    }
}
