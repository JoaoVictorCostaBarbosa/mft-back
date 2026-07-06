use crate::application::dtos::workout_template::WorkoutTemplateRequest;
use crate::application::errors::AppError;
use crate::domain::entities::Exercise;
use crate::domain::entities::User;
use crate::domain::entities::WorkoutTemplate;
use crate::domain::repositories::WorkoutTemplateRepository;
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
    ) -> Result<WorkoutTemplate, AppError> {
        let exercises: Vec<Exercise> = vec![];
        let workout = WorkoutTemplate::new(current_user.id, workout_request.name, exercises)?;

        self.workout_repo.save(&workout).await?;

        Ok(workout)
    }
}
