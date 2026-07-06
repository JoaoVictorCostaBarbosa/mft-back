use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::repositories::WorkoutPlanRepository;
use crate::domain::repositories::WorkoutTemplateRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct RemoveWorkoutTemplateFromWorkoutPlan {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
    workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl RemoveWorkoutTemplateFromWorkoutPlan {
    pub fn new(
        workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
        workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
    ) -> Self {
        Self {
            workout_plan_repo,
            workout_template_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_plan_id: Uuid,
        workout_template_id: Uuid,
    ) -> Result<(), AppError> {
        let mut workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;

        workout_plan.assert_owner(&current_user)?;

        let workout_template = self
            .workout_template_repo
            .find_by_id(workout_template_id)
            .await?;

        workout_template.assert_owner(&current_user)?;

        workout_plan.remove_workout_template(workout_template_id)?;

        self.workout_plan_repo
            .remove_workout_template(workout_plan_id, workout_template_id)
            .await?;

        Ok(())
    }
}
