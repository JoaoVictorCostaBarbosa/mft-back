use crate::application::dtos::exercise::UpdateExerciseInput;
use crate::application::errors::AppError;
use crate::domain::commands::ExerciseUpdateFields;
use crate::domain::entities::User;
use crate::domain::enums::Role;
use crate::domain::errors::ExerciseError;
use crate::domain::errors::UserError;
use crate::domain::repositories::ExerciseRepository;
use crate::domain::value_objects::Name;
use std::sync::Arc;

pub struct UpdateExercise {
    exercise_repo: Arc<dyn ExerciseRepository>,
}

impl UpdateExercise {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        input: UpdateExerciseInput,
    ) -> Result<(), AppError> {
        if input.is_empty() {
            return Err(ExerciseError::InvalidFieldsCriteria.into());
        }

        if let Some(n) = input.name.clone() {
            Name::new(n).map_err(UserError::from)?;
        }

        let user_id = match current_user.role {
            Role::Admin => None,
            Role::User => Some(current_user.id),
        };

        let fields = ExerciseUpdateFields {
            id: input.id,
            name: input.name,
            exercise_type: input.exercise_type,
            equipment: input.equipment,
            muscle_group: input.muscle_group,
        };

        self.exercise_repo.update_exercise(fields, user_id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::DomainError;
    use crate::test_support::fakes::InMemoryExerciseRepository;
    use crate::test_support::fixtures;
    use uuid::Uuid;

    fn empty_input() -> UpdateExerciseInput {
        UpdateExerciseInput {
            id: Uuid::new_v4(),
            name: None,
            exercise_type: None,
            equipment: None,
            muscle_group: None,
        }
    }

    #[tokio::test]
    async fn rejects_update_without_fields() {
        let repo = Arc::new(InMemoryExerciseRepository::default());
        let use_case = UpdateExercise::new(repo.clone());

        let err = use_case
            .execute(fixtures::user(), empty_input())
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Exercise(ExerciseError::InvalidFieldsCriteria))
        ));
        assert!(repo.updates.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn regular_user_update_is_scoped_to_his_exercises() {
        let repo = Arc::new(InMemoryExerciseRepository::default());
        let use_case = UpdateExercise::new(repo.clone());
        let user = fixtures::user();
        let user_id = user.id;
        let input = UpdateExerciseInput {
            name: Some("Novo nome".to_string()),
            ..empty_input()
        };

        use_case.execute(user, input).await.unwrap();

        let updates = repo.updates.lock().unwrap();
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].1, Some(user_id));
    }

    #[tokio::test]
    async fn admin_update_is_not_scoped() {
        let repo = Arc::new(InMemoryExerciseRepository::default());
        let use_case = UpdateExercise::new(repo.clone());
        let input = UpdateExerciseInput {
            name: Some("Novo nome".to_string()),
            ..empty_input()
        };

        use_case.execute(fixtures::admin(), input).await.unwrap();

        let updates = repo.updates.lock().unwrap();
        assert_eq!(updates[0].1, None);
    }
}
