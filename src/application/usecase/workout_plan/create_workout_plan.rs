use crate::application::dtos::WorkoutPlanRequest;
use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutPlan;
use crate::domain::repositories::WorkoutPlanRepository;
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
    ) -> Result<WorkoutPlan, AppError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::enums::RoutineMode;
    use crate::domain::errors::DomainError;
    use crate::test_support::fakes::FakeWorkoutPlanRepository;
    use crate::test_support::fixtures;
    use std::sync::Arc;
    use uuid::Uuid;

    fn repo() -> Arc<FakeWorkoutPlanRepository> {
        Arc::new(FakeWorkoutPlanRepository::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
        ))
    }

    #[tokio::test]
    async fn creates_plan_for_current_user() {
        let repo = repo();
        let use_case = CreateWorkoutPlan::new(repo.clone());
        let user = fixtures::user();
        let user_id = user.id;

        let plan = use_case
            .execute(
                user,
                WorkoutPlanRequest {
                    name: "Treino A".to_string(),
                    routine_mode: RoutineMode::Weekly,
                },
            )
            .await
            .unwrap();

        assert_eq!(plan.user_id, user_id);
        assert_eq!(repo.saved_plans.lock().unwrap().as_slice(), &["Treino A"]);
    }

    #[tokio::test]
    async fn rejects_invalid_name() {
        let repo = repo();
        let use_case = CreateWorkoutPlan::new(repo.clone());

        let err = match use_case
            .execute(
                fixtures::user(),
                WorkoutPlanRequest {
                    name: String::new(),
                    routine_mode: RoutineMode::Weekly,
                },
            )
            .await
        {
            Ok(_) => panic!("expected error"),
            Err(err) => err,
        };

        assert!(matches!(err, AppError::Domain(DomainError::WorkoutPlan(_))));
        assert!(repo.saved_plans.lock().unwrap().is_empty());
    }
}
