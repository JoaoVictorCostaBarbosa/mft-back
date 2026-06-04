use crate::domain::{
    entities::{user::User, workout_plan::WorkoutPlanRoutineItem},
    enums::{day_of_week::DayOfWeek, routine_item_type::RoutineItemType},
    errors::domain_error::DomainError,
    repositories::{
        workout_plan_repository::WorkoutPlanRepository,
        workout_template_repository::WorkoutTemplateRepository,
    },
};
use std::sync::Arc;
use uuid::Uuid;

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
        workout_plan_id: Uuid,
        routine_item_id: Uuid,
        item_type: Option<RoutineItemType>,
        workout_template_id: Option<Uuid>,
        day_of_week: Option<DayOfWeek>,
        position: Option<u32>,
    ) -> Result<WorkoutPlanRoutineItem, DomainError> {
        let mut workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;

        workout_plan.assert_owner(&current_user)?;

        let workout_template = match workout_template_id {
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
            routine_item_id,
            item_type,
            workout_template,
            day_of_week,
            position,
        )?;

        self.workout_plan_repo
            .update_routine_item(&routine_item, workout_plan_id)
            .await?;

        Ok(routine_item)
    }
}
