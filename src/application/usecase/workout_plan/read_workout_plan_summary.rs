use crate::application::errors::AppError;
use crate::domain::commands::WorkoutPlanFilterFields;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutPlanSummary;
use crate::domain::repositories::WorkoutPlanRepository;
use std::sync::Arc;

pub struct ReadWorkoutPlanSummary {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
}

impl ReadWorkoutPlanSummary {
    pub fn new(workout_plan_repo: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self { workout_plan_repo }
    }

    pub async fn execute(&self, current_user: User) -> Result<Vec<WorkoutPlanSummary>, AppError> {
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
