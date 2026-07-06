use crate::application::errors::AppError;
use crate::domain::commands::ExerciseFilterFields;
use crate::domain::entities::Exercise;
use crate::domain::entities::User;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::ExerciseRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetExerciseById {
    exercise_repo: Arc<dyn ExerciseRepository>,
}

impl GetExerciseById {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(
        &self,
        exercise_id: Uuid,
        current_user: User,
    ) -> Result<Exercise, AppError> {
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
