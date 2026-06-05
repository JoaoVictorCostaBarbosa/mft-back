use crate::domain::{
    entities::{user::User, workout_session::WorkoutSessionExercise},
    errors::{domain_error::DomainError, workout_log_error::WorkoutLogError},
    repositories::workout_session_repository::WorkoutSessionRepository,
};
use std::sync::Arc;
use uuid::Uuid;

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
        session_id: Uuid,
        exercise_id: Uuid,
    ) -> Result<WorkoutSessionExercise, DomainError> {
        let session = self.workout_session_repo.find_by_id(session_id).await?;

        if session.user_id != current_user.id {
            return Err(WorkoutLogError::Forbidden.into());
        }

        self.workout_session_repo
            .add_exercise(session_id, exercise_id)
            .await
    }
}
