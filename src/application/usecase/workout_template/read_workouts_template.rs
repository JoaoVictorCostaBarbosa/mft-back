use crate::application::errors::AppError;
use crate::domain::commands::WorkoutTemplateFilterFields;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutTemplateSummary;
use crate::domain::repositories::WorkoutTemplateRepository;
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
    ) -> Result<Vec<WorkoutTemplateSummary>, AppError> {
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
