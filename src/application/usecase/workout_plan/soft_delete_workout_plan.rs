use crate::domain::{
    entities::user::User, errors::domain_error::DomainError,
    repositories::workout_plan_repository::WorkoutPlanRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct SoftDeleteWorkoutPlan {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
}

impl SoftDeleteWorkoutPlan {
    pub fn new(workout_plan_repo: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self { workout_plan_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_plan_id: Uuid,
    ) -> Result<(), DomainError> {
        let workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;

        workout_plan.assert_owner(&current_user)?;

        self.workout_plan_repo.soft_delete(workout_plan_id).await?;

        Ok(())
    }
}
