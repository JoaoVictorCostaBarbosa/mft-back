use crate::{
    application::dtos::workout_template::workout_template_request::WorkoutTemplateRequest,
    domain::{
        entities::{exercise::Exercise, user::User, workout_template::WorkoutTemplate},
        errors::domain_error::DomainError,
        repositories::workout_template_repository::WorkoutTemplateRepository,
    },
};
use std::sync::Arc;

pub struct CreateWorkoutTemplate {
    workout_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl CreateWorkoutTemplate {
    pub fn new(workout_repo: Arc<dyn WorkoutTemplateRepository>) -> Self {
        Self { workout_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        workout_request: WorkoutTemplateRequest,
    ) -> Result<WorkoutTemplate, DomainError> {
        let exercises: Vec<Exercise> = vec![];
        let workout = WorkoutTemplate::new(current_user.id, workout_request.name, exercises)?;

        self.workout_repo.save(&workout).await?;

        Ok(workout)
    }
}
