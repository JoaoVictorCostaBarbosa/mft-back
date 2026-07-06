use crate::application::dtos::workout_template::WorkoutTemplateExerciseRequest;
use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::repositories::ExerciseRepository;
use crate::domain::repositories::WorkoutTemplateRepository;
use std::sync::Arc;

pub struct AddExerciseToWorkoutTemplate {
    exercise_repo: Arc<dyn ExerciseRepository>,
    workout_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl AddExerciseToWorkoutTemplate {
    pub fn new(
        exercise_repo: Arc<dyn ExerciseRepository>,
        workout_repo: Arc<dyn WorkoutTemplateRepository>,
    ) -> Self {
        Self {
            exercise_repo,
            workout_repo,
        }
    }

    pub async fn execute(
        &self,
        current_user: User,
        request: WorkoutTemplateExerciseRequest,
    ) -> Result<(), AppError> {
        let mut workout = self.workout_repo.find_by_id(request.workout_id).await?;

        workout.assert_owner(&current_user)?;

        let exercise = self.exercise_repo.read_by_id(request.exercise_id).await?;

        workout.add_exercise(exercise)?;

        self.workout_repo
            .add_exercise(request.workout_id, request.exercise_id)
            .await?;

        Ok(())
    }
}
