use crate::application::dtos::workout_template::WorkoutTemplateUpdateRequest;
use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutTemplate;
use crate::domain::repositories::WorkoutTemplateRepository;
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
    ) -> Result<WorkoutTemplate, AppError> {
        let fields = WorkoutTemplateUpdateRequest { workout_id, name };

        let mut workout = self.workout_repo.find_by_id(fields.workout_id).await?;

        workout.assert_owner(&current_user)?;

        workout.update_template(fields.name)?;

        self.workout_repo.update(&workout).await?;

        Ok(workout)
    }
}
