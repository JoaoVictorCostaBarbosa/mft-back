use crate::domain::{
    entities::user::User, errors::domain_error::DomainError,
    repositories::workout_template_repository::WorkoutTemplateRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteWorkoutTemplate {
    workout_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl DeleteWorkoutTemplate {
    pub fn new(workout_repo: Arc<dyn WorkoutTemplateRepository>) -> Self {
        Self { workout_repo }
    }

    pub async fn execute(&self, current_user: User, workout_id: Uuid) -> Result<(), DomainError> {
        let workout = self.workout_repo.find_by_id(workout_id).await?;

        workout.assert_owner(&current_user)?;

        self.workout_repo.delete(workout.id).await?;

        Ok(())
    }
}
