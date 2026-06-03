use crate::domain::commands::exercise_commands::ExerciseFilterFields;
use crate::domain::{
    entities::{exercise::Exercise, user::User},
    errors::{domain_error::DomainError, repository_error::RepositoryError},
    repositories::exercise_repository::ExerciseRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetExerciseById {
    pub exercise_repo: Arc<dyn ExerciseRepository>,
}

impl GetExerciseById {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(
        &self,
        exercise_id: Uuid,
        current_user: User,
    ) -> Result<Exercise, DomainError> {
        let exercises = self
            .exercise_repo
            .get_exercises(ExerciseFilterFields {
                id: Some(exercise_id),
                user_id: Some(current_user.id),
                ..Default::default()
            })
            .await?;

        match exercises.items.first() {
            Some(exercice) => Ok(exercice.to_owned()),
            None => Err(RepositoryError::NotFound("exercise not found".to_string()).into()),
        }
    }
}
