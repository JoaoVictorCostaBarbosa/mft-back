use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::repositories::WorkoutPlanRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct RemoveRoutineItem {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
}

impl RemoveRoutineItem {
    pub fn new(workout_plan_repo: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self { workout_plan_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_plan_id: Uuid,
        routine_item_id: Uuid,
    ) -> Result<(), AppError> {
        let mut workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;

        workout_plan.assert_owner(&current_user)?;
        workout_plan.remove_routine_item(routine_item_id)?;

        self.workout_plan_repo
            .remove_routine_item(workout_plan_id, routine_item_id)
            .await?;

        Ok(())
    }
}
