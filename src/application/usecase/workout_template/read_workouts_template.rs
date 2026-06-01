use crate::domain::{
    commands::workout_template_command::WorkoutTemplateFilterFields,
    entities::{user::User, workout_template::WorkoutTemplateSummary},
    errors::domain_error::DomainError,
    repositories::workout_template_repository::WorkoutTemplateRepository,
};
use std::sync::Arc;

pub struct ReadWorkoutsTemplate {
    workout_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl ReadWorkoutsTemplate {
    pub fn new(workout_repo: Arc<dyn WorkoutTemplateRepository>) -> Self {
        Self { workout_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
    ) -> Result<Vec<WorkoutTemplateSummary>, DomainError> {
        let workouts = self
            .workout_repo
            .read(WorkoutTemplateFilterFields {
                user_id: current_user.id,
                ..Default::default()
            })
            .await?;

        Ok(workouts)
    }
}
