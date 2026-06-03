use crate::domain::{
    entities::user::User,
    enums::{day_of_week::DayOfWeek, routine_item_type::RoutineItemType},
    errors::domain_error::DomainError,
    repositories::{
        workout_plan_repository::WorkoutPlanRepository,
        workout_template_repository::WorkoutTemplateRepository,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct AddWorkoutTemplateToWorkoutPlan {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
    workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl AddWorkoutTemplateToWorkoutPlan {
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
        workout_template_id: Option<Uuid>,
        item_type: RoutineItemType,
        day_of_week: Option<DayOfWeek>,
        position: Option<u32>,
    ) -> Result<(), DomainError> {
        let mut workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;

        workout_plan.assert_owner(&current_user)?;

        let workout_template = match workout_template_id {
            Some(workout_template_id) => {
                let workout_template = self
                    .workout_template_repo
                    .find_by_id(workout_template_id)
                    .await?; // TODO: atualmente ele carrega todos os exercises do template.

                workout_template.assert_owner(&current_user)?;

                Some(workout_template)
            }
            None => None,
        };

        workout_plan.add_routine_item(item_type, workout_template, day_of_week, position)?;

        let routine_item = workout_plan
            .routine_items
            .last()
            .expect("routine item was just added");

        self.workout_plan_repo
            .add_routine_item(routine_item, workout_plan_id)
            .await?;

        Ok(())
    }
}
