use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutSession;
use crate::domain::errors::WorkoutSessionError;
use crate::domain::repositories::WorkoutPlanRepository;
use crate::domain::repositories::WorkoutSessionRepository;
use crate::domain::repositories::WorkoutTemplateRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct StartWorkoutSession {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
    workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
    workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl StartWorkoutSession {
    pub fn new(
        workout_session_repo: Arc<dyn WorkoutSessionRepository>,
        workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
        workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
    ) -> Self {
        Self {
            workout_session_repo,
            workout_plan_repo,
            workout_template_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_plan_id: Option<Uuid>,
        workout_template_id: Option<Uuid>,
    ) -> Result<WorkoutSession, AppError> {
        if self
            .workout_session_repo
            .has_in_progress(current_user.id)
            .await?
        {
            return Err(WorkoutSessionError::AlreadyInProgress.into());
        }

        if let Some(workout_plan_id) = workout_plan_id {
            let workout_plan = self.workout_plan_repo.find_by_id(workout_plan_id).await?;
            workout_plan.assert_owner(&current_user)?;
        }

        if let Some(workout_template_id) = workout_template_id {
            let workout_template = self
                .workout_template_repo
                .find_by_id(workout_template_id)
                .await?;
            workout_template.assert_owner(&current_user)?;
        }

        let session = WorkoutSession::start(current_user.id, workout_plan_id, workout_template_id);
        self.workout_session_repo.start(&session).await?;

        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::enums::WorkoutSessionStatus;
    use crate::domain::errors::DomainError;
    use crate::domain::errors::PermissionError;
    use crate::test_support::fakes::FakeWorkoutPlanRepository;
    use crate::test_support::fakes::FakeWorkoutTemplateRepository;
    use crate::test_support::fakes::InMemoryWorkoutSessionRepository;
    use crate::test_support::fixtures;

    #[tokio::test]
    async fn starts_session_for_owned_plan_and_template() {
        let user = fixtures::user();
        let plan_id = Uuid::new_v4();
        let template_id = Uuid::new_v4();
        let session_repo = Arc::new(InMemoryWorkoutSessionRepository::default());
        let use_case = StartWorkoutSession::new(
            session_repo.clone(),
            Arc::new(FakeWorkoutPlanRepository::new(plan_id, user.id)),
            Arc::new(FakeWorkoutTemplateRepository {
                template_id,
                owner_id: user.id,
            }),
        );

        let session = use_case
            .execute(user, Some(plan_id), Some(template_id))
            .await
            .unwrap();

        assert_eq!(session.status, WorkoutSessionStatus::InProgress);
        assert_eq!(session_repo.sessions.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn starts_free_session_without_plan_and_template() {
        let user = fixtures::user();
        let session_repo = Arc::new(InMemoryWorkoutSessionRepository::default());
        let use_case = StartWorkoutSession::new(
            session_repo.clone(),
            Arc::new(FakeWorkoutPlanRepository::new(Uuid::new_v4(), user.id)),
            Arc::new(FakeWorkoutTemplateRepository {
                template_id: Uuid::new_v4(),
                owner_id: user.id,
            }),
        );

        let session = use_case.execute(user, None, None).await.unwrap();

        assert_eq!(session.status, WorkoutSessionStatus::InProgress);
        assert!(session.workout_plan_id.is_none());
        assert!(session.workout_template_id.is_none());
        assert_eq!(session_repo.sessions.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn rejects_when_session_already_in_progress() {
        let user = fixtures::user();
        let plan_id = Uuid::new_v4();
        let template_id = Uuid::new_v4();
        let existing = fixtures::in_progress_session(user.id);
        let session_repo = Arc::new(InMemoryWorkoutSessionRepository::with_sessions(vec![
            existing,
        ]));
        let use_case = StartWorkoutSession::new(
            session_repo.clone(),
            Arc::new(FakeWorkoutPlanRepository::new(plan_id, user.id)),
            Arc::new(FakeWorkoutTemplateRepository {
                template_id,
                owner_id: user.id,
            }),
        );

        let err = use_case
            .execute(user, Some(plan_id), Some(template_id))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::WorkoutSession(
                WorkoutSessionError::AlreadyInProgress
            ))
        ));
        assert_eq!(session_repo.sessions.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn rejects_plan_owned_by_another_user() {
        let user = fixtures::user();
        let plan_id = Uuid::new_v4();
        let template_id = Uuid::new_v4();
        let session_repo = Arc::new(InMemoryWorkoutSessionRepository::default());
        let use_case = StartWorkoutSession::new(
            session_repo.clone(),
            Arc::new(FakeWorkoutPlanRepository::new(plan_id, Uuid::new_v4())),
            Arc::new(FakeWorkoutTemplateRepository {
                template_id,
                owner_id: user.id,
            }),
        );

        let err = use_case
            .execute(user, Some(plan_id), Some(template_id))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
        assert!(session_repo.sessions.lock().unwrap().is_empty());
    }
}
