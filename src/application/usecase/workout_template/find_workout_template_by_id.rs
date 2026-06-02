use crate::domain::{
    entities::{user::User, workout_template::WorkoutTemplate},
    errors::domain_error::DomainError,
    repositories::workout_template_repository::WorkoutTemplateRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct FindWorkoutTemplateById {
    workout_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl FindWorkoutTemplateById {
    pub fn new(workout_repo: Arc<dyn WorkoutTemplateRepository>) -> Self {
        Self { workout_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        wt_id: Uuid,
    ) -> Result<WorkoutTemplate, DomainError> {
        let template = self.workout_repo.find_by_id(wt_id).await?;

        template.assert_owner(&current_user)?;

        Ok(template)
    }
}
