use crate::application::dtos::AddRoutineItemInput;
use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::repositories::WorkoutPlanRepository;
use crate::domain::repositories::WorkoutTemplateRepository;
use std::sync::Arc;

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
        input: AddRoutineItemInput,
    ) -> Result<(), AppError> {
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
                    .await?; // TODO: atualmente ele carrega todos os exercises do template.

                workout_template.assert_owner(&current_user)?;

                Some(workout_template)
            }
            None => None,
        };

        workout_plan.add_routine_item(
            input.item_type,
            workout_template,
            input.day_of_week,
            input.position,
        )?;

        let routine_item = workout_plan
            .routine_items
            .last()
            .expect("routine item was just added");

        self.workout_plan_repo
            .add_routine_item(routine_item, input.workout_plan_id)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::enums::DayOfWeek;
    use crate::domain::enums::RoutineItemType;
    use crate::domain::errors::DomainError;
    use crate::domain::errors::PermissionError;
    use crate::domain::errors::WorkoutPlanError;
    use crate::test_support::fakes::FakeWorkoutPlanRepository;
    use crate::test_support::fakes::FakeWorkoutTemplateRepository;
    use crate::test_support::fixtures;
    use uuid::Uuid;

    fn input(workout_plan_id: Uuid, day_of_week: Option<DayOfWeek>) -> AddRoutineItemInput {
        AddRoutineItemInput {
            workout_plan_id,
            workout_template_id: None,
            item_type: RoutineItemType::Rest,
            day_of_week,
            position: None,
        }
    }

    fn template_repo(owner_id: Uuid) -> Arc<FakeWorkoutTemplateRepository> {
        Arc::new(FakeWorkoutTemplateRepository {
            template_id: Uuid::new_v4(),
            owner_id,
        })
    }

    #[tokio::test]
    async fn weekly_plan_requires_day_of_week() {
        let user = fixtures::user();
        let plan_id = Uuid::new_v4();
        let plan_repo = Arc::new(FakeWorkoutPlanRepository::new(plan_id, user.id));
        let use_case =
            AddWorkoutTemplateToWorkoutPlan::new(plan_repo.clone(), template_repo(user.id));

        let err = use_case
            .execute(user, input(plan_id, None))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::WorkoutPlan(
                WorkoutPlanError::WeeklyRoutineRequiresDayOfWeek
            ))
        ));
        assert!(plan_repo.added_routine_items.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn adds_rest_item_to_weekly_plan() {
        let user = fixtures::user();
        let plan_id = Uuid::new_v4();
        let plan_repo = Arc::new(FakeWorkoutPlanRepository::new(plan_id, user.id));
        let use_case =
            AddWorkoutTemplateToWorkoutPlan::new(plan_repo.clone(), template_repo(user.id));

        use_case
            .execute(user, input(plan_id, Some(DayOfWeek::Monday)))
            .await
            .unwrap();

        assert_eq!(
            plan_repo.added_routine_items.lock().unwrap().as_slice(),
            &[plan_id]
        );
    }

    #[tokio::test]
    async fn non_owner_is_forbidden() {
        let plan_id = Uuid::new_v4();
        let plan_repo = Arc::new(FakeWorkoutPlanRepository::new(plan_id, Uuid::new_v4()));
        let use_case =
            AddWorkoutTemplateToWorkoutPlan::new(plan_repo.clone(), template_repo(Uuid::new_v4()));

        let err = use_case
            .execute(fixtures::user(), input(plan_id, Some(DayOfWeek::Monday)))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
    }
}
