use crate::domain::{
    commands::workout_plan_command::WorkoutPlanFilterFields,
    entities::{user::User, workout_plan::WorkoutPlanSummary},
    errors::domain_error::DomainError,
    repositories::workout_plan_repository::WorkoutPlanRepository,
};
use std::sync::Arc;

pub struct ReadWorkoutPlanSummary {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
}

impl ReadWorkoutPlanSummary {
    pub fn new(workout_plan_repo: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self { workout_plan_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
    ) -> Result<Vec<WorkoutPlanSummary>, DomainError> {
        let workout_plans = self
            .workout_plan_repo
            .read_summary(WorkoutPlanFilterFields {
                user_id: current_user.id,
                ..Default::default()
            })
            .await?;

        Ok(workout_plans)
    }
}
