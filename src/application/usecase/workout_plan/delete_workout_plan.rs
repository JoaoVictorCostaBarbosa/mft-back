use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::repositories::WorkoutPlanRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteWorkoutPlan {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
}

impl DeleteWorkoutPlan {
    pub fn new(workout_plan_repo: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self { workout_plan_repo }
    }

    pub async fn execute(&self, current_user: User, workout_plan_id: Uuid) -> Result<(), AppError> {
        let workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;

        workout_plan.assert_owner(&current_user)?;

        self.workout_plan_repo.delete(workout_plan_id).await?;

        Ok(())
    }
}
