use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::errors::PermissionError;
use crate::domain::repositories::WorkoutSessionRepository;
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

    pub async fn execute(&self, current_user: User, session_id: Uuid) -> Result<(), AppError> {
        let session = self.workout_session_repo.find_by_id(session_id).await?;

        if session.user_id != current_user.id {
            return Err(PermissionError::Forbidden.into());
        }

        session.assert_in_progress()?;
        Ok(self.workout_session_repo.cancel(session_id).await?)
    }
}
