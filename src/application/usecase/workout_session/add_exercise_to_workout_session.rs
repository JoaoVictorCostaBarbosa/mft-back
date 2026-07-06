use crate::application::dtos::workout_session::AddExerciseToSessionInput;
use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutSessionExercise;
use crate::domain::errors::PermissionError;
use crate::domain::repositories::WorkoutSessionRepository;
use std::sync::Arc;

pub struct AddExerciseToWorkoutSession {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
}

impl AddExerciseToWorkoutSession {
    pub fn new(workout_session_repo: Arc<dyn WorkoutSessionRepository>) -> Self {
        Self {
            workout_session_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        input: AddExerciseToSessionInput,
    ) -> Result<WorkoutSessionExercise, AppError> {
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
            .add_exercise(
                input.session_id,
                input.exercise_id,
                input.client_operation_id,
            )
            .await?)
    }
}
