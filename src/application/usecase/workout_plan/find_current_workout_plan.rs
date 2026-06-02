use std::sync::Arc;

use crate::domain::{
    entities::{user::User, workout_plan::WorkoutPlan},
    errors::domain_error::DomainError,
    repositories::workout_plan_repository::WorkoutPlanRepository,
};

pub struct FindCurrentWorkoutPlan {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
}

impl FindCurrentWorkoutPlan {
    pub fn new(workout_plan_repo: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self { workout_plan_repo }
    }

    pub async fn execute(&self, current_user: User) -> Result<WorkoutPlan, DomainError> {
        let workout_plan = self
            .workout_plan_repo
            .find_current_user_plan(current_user.id)
            .await?;

        Ok(workout_plan)
    }
}
