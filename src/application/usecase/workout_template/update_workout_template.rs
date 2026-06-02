use crate::{
    application::dtos::workout_template::workout_template_update_request::WorkoutTemplateUpdateRequest,
    domain::{
        entities::{user::User, workout_template::WorkoutTemplate},
        errors::domain_error::DomainError,
        repositories::workout_template_repository::WorkoutTemplateRepository,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateWorkoutTemplate {
    workout_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl UpdateWorkoutTemplate {
    pub fn new(workout_repo: Arc<dyn WorkoutTemplateRepository>) -> Self {
        Self { workout_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_id: Uuid,
        name: Option<String>,
    ) -> Result<WorkoutTemplate, DomainError> {
        let fields = WorkoutTemplateUpdateRequest { workout_id, name };

        let mut workout = self.workout_repo.find_by_id(fields.workout_id).await?;

        workout.assert_owner(&current_user)?;

        workout.update_template(fields.name)?;

        self.workout_repo.update(&workout).await?;

        Ok(workout)
    }
}
