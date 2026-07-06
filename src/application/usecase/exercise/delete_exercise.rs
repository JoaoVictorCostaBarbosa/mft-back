use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::enums::Role;
use crate::domain::errors::PermissionError;
use crate::domain::repositories::ExerciseRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteExercise {
    exercise_repo: Arc<dyn ExerciseRepository>,
}

impl DeleteExercise {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(&self, id: Uuid, current_user: User) -> Result<(), AppError> {
        if current_user.role != Role::Admin {
            return Err(PermissionError::Forbidden.into());
        }

        self.exercise_repo.delete_exercise(id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::DomainError;
    use crate::test_support::fakes::InMemoryExerciseRepository;
    use crate::test_support::fixtures;

    #[tokio::test]
    async fn non_admin_cannot_delete() {
        let repo = Arc::new(InMemoryExerciseRepository::default());
        let use_case = DeleteExercise::new(repo.clone());

        let err = use_case
            .execute(Uuid::new_v4(), fixtures::user())
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
        assert!(repo.deleted.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn admin_deletes_exercise() {
        let repo = Arc::new(InMemoryExerciseRepository::default());
        let use_case = DeleteExercise::new(repo.clone());
        let id = Uuid::new_v4();

        use_case.execute(id, fixtures::admin()).await.unwrap();

        assert_eq!(repo.deleted.lock().unwrap().as_slice(), &[id]);
    }
}
