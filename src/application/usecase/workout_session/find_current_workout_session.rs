use crate::application::errors::AppError;
use crate::application::ports::WorkoutSessionQueries;
use crate::application::read_models::CurrentWorkoutSession;
use crate::domain::entities::User;
use std::sync::Arc;

pub struct FindCurrentWorkoutSession {
    workout_session_queries: Arc<dyn WorkoutSessionQueries>,
}

impl FindCurrentWorkoutSession {
    pub fn new(workout_session_queries: Arc<dyn WorkoutSessionQueries>) -> Self {
        Self {
            workout_session_queries,
        }
    }

    pub async fn execute(&self, current_user: User) -> Result<CurrentWorkoutSession, AppError> {
        Ok(self
            .workout_session_queries
            .find_current(current_user.id)
            .await?)
    }
}
