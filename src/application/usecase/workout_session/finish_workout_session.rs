use crate::application::dtos::workout_session::FinishWorkoutSessionInput;
use crate::application::errors::AppError;
use crate::domain::entities::FinishedWorkoutSession;
use crate::domain::entities::User;
use crate::domain::errors::PermissionError;
use crate::domain::repositories::WorkoutSessionRepository;
use std::sync::Arc;

pub struct FinishWorkoutSession {
    workout_session_repo: Arc<dyn WorkoutSessionRepository>,
}

impl FinishWorkoutSession {
    pub fn new(workout_session_repo: Arc<dyn WorkoutSessionRepository>) -> Self {
        Self {
            workout_session_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        input: FinishWorkoutSessionInput,
    ) -> Result<FinishedWorkoutSession, AppError> {
        let session = self
            .workout_session_repo
            .find_by_id(input.session_id)
            .await?;

        if session.user_id != current_user.id {
            return Err(PermissionError::Forbidden.into());
        }

        let finished = session.finish(input.finished_at)?;
        self.workout_session_repo.finish(&finished).await?;

        Ok(finished)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::enums::WorkoutSessionStatus;
    use crate::domain::errors::DomainError;
    use crate::domain::errors::WorkoutSessionError;
    use crate::test_support::fakes::InMemoryWorkoutSessionRepository;
    use crate::test_support::fixtures;

    #[tokio::test]
    async fn owner_finishes_own_session() {
        let user = fixtures::user();
        let session = fixtures::in_progress_session(user.id);
        let session_id = session.id;
        let repo = Arc::new(InMemoryWorkoutSessionRepository::with_sessions(vec![
            session,
        ]));
        let use_case = FinishWorkoutSession::new(repo.clone());

        let finished = use_case
            .execute(
                user,
                FinishWorkoutSessionInput {
                    session_id,
                    finished_at: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(finished.status, WorkoutSessionStatus::Finished);
        assert_eq!(repo.finished.lock().unwrap().len(), 1);
        assert_eq!(
            repo.sessions.lock().unwrap()[0].status,
            WorkoutSessionStatus::Finished
        );
    }

    #[tokio::test]
    async fn other_user_cannot_finish_session() {
        let owner = fixtures::user();
        let intruder = fixtures::user_with_email("intruder@test.com");
        let session = fixtures::in_progress_session(owner.id);
        let session_id = session.id;
        let repo = Arc::new(InMemoryWorkoutSessionRepository::with_sessions(vec![
            session,
        ]));
        let use_case = FinishWorkoutSession::new(repo.clone());

        let err = use_case
            .execute(
                intruder,
                FinishWorkoutSessionInput {
                    session_id,
                    finished_at: None,
                },
            )
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
        assert!(repo.finished.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn finished_session_cannot_be_finished_again() {
        let user = fixtures::user();
        let mut session = fixtures::in_progress_session(user.id);
        session.status = WorkoutSessionStatus::Finished;
        let session_id = session.id;
        let repo = Arc::new(InMemoryWorkoutSessionRepository::with_sessions(vec![
            session,
        ]));
        let use_case = FinishWorkoutSession::new(repo);

        let err = use_case
            .execute(
                user,
                FinishWorkoutSessionInput {
                    session_id,
                    finished_at: None,
                },
            )
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::WorkoutSession(
                WorkoutSessionError::AlreadyFinished
            ))
        ));
    }
}
