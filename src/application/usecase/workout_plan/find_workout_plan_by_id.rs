use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutPlan;
use crate::domain::repositories::WorkoutPlanRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct FindWorkoutPlanById {
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
}

impl FindWorkoutPlanById {
    pub fn new(workout_plan_repo: Arc<dyn WorkoutPlanRepository>) -> Self {
        Self { workout_plan_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_plan_id: Uuid,
    ) -> Result<WorkoutPlan, AppError> {
        let workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;

        workout_plan.assert_owner(&current_user)?;

        Ok(workout_plan)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::DomainError;
    use crate::domain::errors::PermissionError;
    use crate::test_support::fakes::FakeWorkoutPlanRepository;
    use crate::test_support::fixtures;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn owner_reads_own_plan() {
        let user = fixtures::user();
        let plan_id = Uuid::new_v4();
        let repo = Arc::new(FakeWorkoutPlanRepository::new(plan_id, user.id));
        let use_case = FindWorkoutPlanById::new(repo);

        let plan = match use_case.execute(user, plan_id).await {
            Ok(plan) => plan,
            Err(err) => panic!("unexpected error: {err}"),
        };

        assert_eq!(plan.id, plan_id);
    }

    #[tokio::test]
    async fn non_owner_is_forbidden() {
        let plan_id = Uuid::new_v4();
        let repo = Arc::new(FakeWorkoutPlanRepository::new(plan_id, Uuid::new_v4()));
        let use_case = FindWorkoutPlanById::new(repo);

        let err = match use_case.execute(fixtures::user(), plan_id).await {
            Ok(_) => panic!("expected error"),
            Err(err) => err,
        };

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
    }
}
