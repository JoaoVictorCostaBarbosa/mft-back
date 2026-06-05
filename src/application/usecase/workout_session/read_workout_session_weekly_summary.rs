use crate::domain::{
    entities::{user::User, workout_session::WorkoutSessionWeeklySummaryDay},
    errors::domain_error::DomainError,
    repositories::workout_session_repository::WorkoutSessionRepository,
};
use chrono::NaiveDate;
use std::sync::Arc;

pub struct ReadWorkoutSessionWeeklySummary {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
}

impl ReadWorkoutSessionWeeklySummary {
    pub fn new(workout_session_repo: Arc<dyn WorkoutSessionRepository>) -> Self {
        Self {
            workout_session_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<WorkoutSessionWeeklySummaryDay>, DomainError> {
        self.workout_session_repo
            .weekly_summary(current_user.id, start_date, end_date)
            .await
    }
}
