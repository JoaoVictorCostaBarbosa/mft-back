use crate::application::dtos::exercise::CreateExerciseRequest;
use crate::application::errors::AppError;
use crate::domain::entities::Exercise;
use crate::domain::entities::User;
use crate::domain::enums::Role;
use crate::domain::repositories::ExerciseRepository;
use std::sync::Arc;

pub struct CreateExercise {
    exercise_repo: Arc<dyn ExerciseRepository>,
}

impl CreateExercise {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(
        &self,
        request: CreateExerciseRequest,
        current_user: User,
    ) -> Result<Exercise, AppError> {
        let user_id = match current_user.role {
            Role::Admin => None,
            _ => Some(current_user.id),
        };

        let exercise = Exercise::new(
            user_id,
            request.name,
            request.exercise_type,
            request.equipment,
            request.muscle_group,
        )?;

        self.exercise_repo.create_exercise(&exercise).await?;

        Ok(exercise)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::enums::{Equipment, ExerciseType, MuscleGroup};
    use crate::domain::errors::DomainError;
    use crate::test_support::fakes::InMemoryExerciseRepository;
    use crate::test_support::fixtures;
    use std::sync::Arc;

    fn request(name: &str) -> CreateExerciseRequest {
        CreateExerciseRequest {
            name: name.to_string(),
            exercise_type: ExerciseType::Strength,
            equipment: Equipment::Barbell,
            muscle_group: MuscleGroup::Chest,
        }
    }

    #[tokio::test]
    async fn user_creates_exercise_owned_by_himself() {
        let repo = Arc::new(InMemoryExerciseRepository::default());
        let use_case = CreateExercise::new(repo.clone());
        let user = fixtures::user();
        let user_id = user.id;

        let exercise = use_case.execute(request("Supino"), user).await.unwrap();

        assert_eq!(exercise.user_id, Some(user_id));
        assert_eq!(repo.exercises.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn admin_creates_global_exercise() {
        let repo = Arc::new(InMemoryExerciseRepository::default());
        let use_case = CreateExercise::new(repo.clone());

        let exercise = use_case
            .execute(request("Supino"), fixtures::admin())
            .await
            .unwrap();

        assert_eq!(exercise.user_id, None);
    }

    #[tokio::test]
    async fn rejects_invalid_name() {
        let repo = Arc::new(InMemoryExerciseRepository::default());
        let use_case = CreateExercise::new(repo.clone());

        let err = use_case
            .execute(request(""), fixtures::user())
            .await
            .unwrap_err();

        assert!(matches!(err, AppError::Domain(DomainError::Exercise(_))));
        assert!(repo.exercises.lock().unwrap().is_empty());
    }
}
