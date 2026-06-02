use crate::domain::{
    entities::{user::User, workout_plan::WorkoutPlan},
    errors::domain_error::DomainError,
    repositories::workout_plan_repository::WorkoutPlanRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct FindWorkoutPlanById {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
}

impl FindWorkoutPlanById {
    pub fn new(workout_plan_repo: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self { workout_plan_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_plan_id: Uuid,
    ) -> Result<WorkoutPlan, DomainError> {
        let workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;

        workout_plan.assert_owner(&current_user)?;

        Ok(workout_plan)
    }
}
