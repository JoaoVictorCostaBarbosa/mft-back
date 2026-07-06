use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutTemplate;
use crate::domain::repositories::WorkoutTemplateRepository;
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
    ) -> Result<WorkoutTemplate, AppError> {
        let template = self.workout_repo.find_by_id(wt_id).await?;

        template.assert_owner(&current_user)?;

        Ok(template)
    }
}
