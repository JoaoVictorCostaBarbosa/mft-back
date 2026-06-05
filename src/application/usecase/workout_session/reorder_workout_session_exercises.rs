use crate::domain::{
    entities::user::User,
    errors::{domain_error::DomainError, workout_log_error::WorkoutLogError},
    repositories::workout_session_repository::WorkoutSessionRepository,
};
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;

pub struct ReorderWorkoutSessionExercises {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
}

impl ReorderWorkoutSessionExercises {
    pub fn new(workout_session_repo: Arc<dyn WorkoutSessionRepository>) -> Self {
        Self {
            workout_session_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        session_id: Uuid,
        ordered_session_exercise_ids: Vec<Uuid>,
    ) -> Result<(), DomainError> {
        if ordered_session_exercise_ids.is_empty() {
            return Err(WorkoutLogError::InvalidExerciseOrder.into());
        }

        let session = self.workout_session_repo.find_by_id(session_id).await?;

        if session.user_id != current_user.id {
            return Err(WorkoutLogError::Forbidden.into());
        }

        session.assert_in_progress()?;

        let current_ids = self
            .workout_session_repo
            .find_session_exercise_ids(session_id)
            .await?;

        let requested_ids: HashSet<Uuid> = ordered_session_exercise_ids.iter().copied().collect();
        let current_ids_set: HashSet<Uuid> = current_ids.iter().copied().collect();

        if requested_ids.len() != ordered_session_exercise_ids.len()
            || requested_ids != current_ids_set
        {
            return Err(WorkoutLogError::InvalidExerciseOrder.into());
        }

        self.workout_session_repo
            .reorder_exercises(session_id, ordered_session_exercise_ids)
            .await
    }
}
