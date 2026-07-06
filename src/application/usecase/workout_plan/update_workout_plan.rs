use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutPlan;
use crate::domain::repositories::WorkoutPlanRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateWorkoutPlan {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
}

impl UpdateWorkoutPlan {
    pub fn new(workout_plan_repo: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self { workout_plan_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_plan_id: Uuid,
        name: Option<String>,
    ) -> Result<WorkoutPlan, AppError> {
        let mut workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;

        workout_plan.assert_owner(&current_user)?;

        workout_plan.update(name)?;

        self.workout_plan_repo.update(&workout_plan).await?;

        Ok(workout_plan)
    }
}
