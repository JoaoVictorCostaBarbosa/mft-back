use crate::application::dtos::workout_session::UpdateSessionSetInput;
use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutSessionSet;
use crate::domain::errors::PermissionError;
use crate::domain::errors::WorkoutSessionError;
use crate::domain::repositories::WorkoutSessionRepository;
use std::sync::Arc;

pub struct UpdateWorkoutSessionSet {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
}

impl UpdateWorkoutSessionSet {
    pub fn new(workout_session_repo: Arc<dyn WorkoutSessionRepository>) -> Self {
        Self {
            workout_session_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        input: UpdateSessionSetInput,
    ) -> Result<WorkoutSessionSet, AppError> {
        if input.reps == 0 {
            return Err(WorkoutSessionError::InvalidReps.into());
        }

        if input.weight < 0.0 {
            return Err(WorkoutSessionError::InvalidWeight.into());
        }

        let session = self
            .workout_session_repo
            .find_by_id(input.session_id)
            .await?;

        if session.user_id != current_user.id {
            return Err(PermissionError::Forbidden.into());
        }

        session.assert_in_progress()?;

        Ok(self
            .workout_session_repo
            .update_set(
                input.session_id,
                input.set_id,
                input.set_type,
                input.weight,
                input.reps,
            )
            .await?)
    }
}
