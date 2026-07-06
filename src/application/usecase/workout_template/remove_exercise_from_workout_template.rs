use crate::application::dtos::workout_template::WorkoutTemplateExerciseRequest;
use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::repositories::WorkoutTemplateRepository;
use std::sync::Arc;

pub struct RemoveExerciseFromWorkoutTemplate {
    workout_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl RemoveExerciseFromWorkoutTemplate {
    pub fn new(workout_repo: Arc<dyn WorkoutTemplateRepository>) -> Self {
        Self { workout_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        request: WorkoutTemplateExerciseRequest,
    ) -> Result<(), AppError> {
        let mut workout = self.workout_repo.find_by_id(request.workout_id).await?;

        workout.assert_owner(&current_user)?;

        workout.remove_exercise(request.exercise_id)?;

        self.workout_repo
            .remove_exercise(request.workout_id, request.exercise_id)
            .await?;

        Ok(())
    }
}
