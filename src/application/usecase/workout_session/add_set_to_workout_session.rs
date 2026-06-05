use crate::domain::{
    entities::{user::User, workout_session::WorkoutSessionSet},
    enums::set_type::SetType,
    errors::{domain_error::DomainError, workout_log_error::WorkoutLogError},
    repositories::workout_session_repository::WorkoutSessionRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct AddSetToWorkoutSession {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
}

impl AddSetToWorkoutSession {
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
        set_type: SetType,
        weight: f32,
        reps: u32,
    ) -> Result<WorkoutSessionSet, DomainError> {
        if reps == 0 {
            return Err(WorkoutLogError::InvalidReps.into());
        }

        if weight < 0.0 {
            return Err(WorkoutLogError::InvalidWeight.into());
        }

        let session = self.workout_session_repo.find_by_id(session_id).await?;

        if session.user_id != current_user.id {
            return Err(WorkoutLogError::Forbidden.into());
        }

        self.workout_session_repo
            .add_set(session_id, exercise_id, set_type, weight, reps)
            .await
    }
}
