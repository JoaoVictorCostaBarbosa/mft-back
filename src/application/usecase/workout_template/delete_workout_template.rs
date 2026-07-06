use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::repositories::WorkoutTemplateRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteWorkoutTemplate {
    workout_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl DeleteWorkoutTemplate {
    pub fn new(workout_repo: Arc<dyn WorkoutTemplateRepository>) -> Self {
        Self { workout_repo }
    }

    pub async fn execute(&self, current_user: User, workout_id: Uuid) -> Result<(), AppError> {
        let workout = self.workout_repo.find_by_id(workout_id).await?;

        workout.assert_owner(&current_user)?;

        self.workout_repo.delete(workout.id).await?;

        Ok(())
    }
}
