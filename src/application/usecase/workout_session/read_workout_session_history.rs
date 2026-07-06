use crate::application::errors::AppError;
use crate::application::ports::WorkoutSessionQueries;
use crate::application::read_models::WorkoutSessionHistoryItem;
use crate::domain::entities::User;
use std::sync::Arc;

pub struct ReadWorkoutSessionHistory {
    workout_session_queries: Arc<dyn WorkoutSessionQueries>,
}

impl ReadWorkoutSessionHistory {
    pub fn new(workout_session_queries: Arc<dyn WorkoutSessionQueries>) -> Self {
        Self {
            workout_session_queries,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
    ) -> Result<Vec<WorkoutSessionHistoryItem>, AppError> {
        Ok(self
            .workout_session_queries
            .history(current_user.id)
            .await?)
    }
}
