use crate::application::errors::AppError;
use crate::application::ports::ExerciseQueries;
use crate::application::read_models::ExerciseLastPerformance;
use crate::domain::entities::User;
use std::sync::Arc;
use uuid::Uuid;

pub struct FindExerciseLastPerformances {
    exercise_queries: Arc<dyn ExerciseQueries>,
}

impl FindExerciseLastPerformances {
    pub fn new(exercise_queries: Arc<dyn ExerciseQueries>) -> Self {
        Self { exercise_queries }
    }

    pub async fn execute(
        &self,
        current_user: User,
        exercise_ids: Vec<Uuid>,
    ) -> Result<Vec<ExerciseLastPerformance>, AppError> {
        if exercise_ids.is_empty() {
            return Ok(Vec::new());
        }

        Ok(self
            .exercise_queries
            .find_last_performances(current_user.id, exercise_ids)
            .await?)
    }
}
