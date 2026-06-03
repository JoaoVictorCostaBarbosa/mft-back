use crate::{
    application::dtos::workout_plan::WorkoutPlanRequest,
    domain::{
        entities::{user::User, workout_plan::WorkoutPlan},
        errors::domain_error::DomainError,
        repositories::workout_plan_repository::WorkoutPlanRepository,
    },
};
use std::sync::Arc;

pub struct CreateWorkoutPlan {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
}

impl CreateWorkoutPlan {
    pub fn new(workout_plan_repo: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self { workout_plan_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_plan_request: WorkoutPlanRequest,
    ) -> Result<WorkoutPlan, DomainError> {
        let workout_plan = WorkoutPlan::new(
            current_user.id,
            workout_plan_request.name,
            workout_plan_request.routine_mode,
            vec![],
        )?;

        self.workout_plan_repo.save(&workout_plan).await?;

        Ok(workout_plan)
    }
}
