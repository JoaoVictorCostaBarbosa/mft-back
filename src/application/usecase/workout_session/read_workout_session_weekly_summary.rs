use crate::application::errors::AppError;
use crate::application::ports::WorkoutSessionQueries;
use crate::application::read_models::WorkoutSessionWeeklySummary;
use crate::domain::entities::User;
use chrono::NaiveDate;
use std::sync::Arc;

pub struct ReadWorkoutSessionWeeklySummary {
    workout_session_queries: Arc<dyn WorkoutSessionQueries>,
}

impl ReadWorkoutSessionWeeklySummary {
    pub fn new(workout_session_queries: Arc<dyn WorkoutSessionQueries>) -> Self {
        Self {
            workout_session_queries,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<WorkoutSessionWeeklySummary, AppError> {
        Ok(self
            .workout_session_queries
            .weekly_summary(current_user.id, start_date, end_date)
            .await?)
    }
}
