use crate::application::errors::AppError;
use crate::application::ports::ExerciseQueries;
use crate::application::read_models::ExercisePersonalRecord;
use crate::domain::entities::User;
use std::sync::Arc;

pub struct ReadPersonalRecords {
    exercise_queries: Arc<dyn ExerciseQueries>,
}

impl ReadPersonalRecords {
    pub fn new(exercise_queries: Arc<dyn ExerciseQueries>) -> Self {
        Self { exercise_queries }
    }

    pub async fn execute(
        &self,
        current_user: User,
    ) -> Result<Vec<ExercisePersonalRecord>, AppError> {
        Ok(self
            .exercise_queries
            .find_personal_records(current_user.id)
            .await?)
    }
}
