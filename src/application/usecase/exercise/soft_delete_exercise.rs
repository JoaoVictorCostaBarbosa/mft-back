use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::repositories::ExerciseRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct SoftDeleteExercise {
    exercise_repo: Arc<dyn ExerciseRepository>,
}

impl SoftDeleteExercise {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(&self, id: Uuid, current_user: User) -> Result<(), AppError> {
        self.exercise_repo
            .soft_delete_exercise(id, current_user.id)
            .await?;

        Ok(())
    }
}
