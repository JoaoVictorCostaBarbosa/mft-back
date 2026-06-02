use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
    entities::user::User, errors::domain_error::DomainError,
    repositories::workout_plan_repository::WorkoutPlanRepository,
};

pub struct SetCurrentWorkoutPlan {
    workout_plan_repository: Arc<dyn WorkoutPlanRepository>,
}

impl SetCurrentWorkoutPlan {
    pub fn new(workout_plan_repository: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self {
            workout_plan_repository,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_plan_id: Uuid,
    ) -> Result<(), DomainError> {
        let workout_plan = self
            .workout_plan_repository
            .find_by_id(workout_plan_id)
            .await?;

        workout_plan.assert_owner(&current_user)?;

        self.workout_plan_repository
            .set_current(current_user.id, workout_plan_id)
            .await?;

        Ok(())
    }
}
