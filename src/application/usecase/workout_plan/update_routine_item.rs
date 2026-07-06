use crate::application::dtos::UpdateRoutineItemInput;
use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutPlanRoutineItem;
use crate::domain::repositories::WorkoutPlanRepository;
use crate::domain::repositories::WorkoutTemplateRepository;
use std::sync::Arc;

pub struct UpdateRoutineItem {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
    workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl UpdateRoutineItem {
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
        input: UpdateRoutineItemInput,
    ) -> Result<WorkoutPlanRoutineItem, AppError> {
        let mut workout_plan = self
            .workout_plan_repo
            .find_by_id(input.workout_plan_id)
            .await?;

        workout_plan.assert_owner(&current_user)?;

        let workout_template = match input.workout_template_id {
            Some(workout_template_id) => {
                let workout_template = self
                    .workout_template_repo
                    .find_by_id(workout_template_id)
                    .await?;

                workout_template.assert_owner(&current_user)?;

                Some(workout_template)
            }
            None => None,
        };

        let routine_item = workout_plan.update_routine_item(
            input.routine_item_id,
            input.item_type,
            workout_template,
            input.day_of_week,
            input.position,
        )?;

        self.workout_plan_repo
            .update_routine_item(&routine_item, input.workout_plan_id)
            .await?;

        Ok(routine_item)
    }
}
