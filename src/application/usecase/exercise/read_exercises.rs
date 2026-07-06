use crate::application::errors::AppError;
use crate::application::pagination::PageRequest;
use crate::domain::commands::ExerciseFilterFields;
use crate::domain::commands::ExercisePaginationFields;
use crate::domain::entities::Exercise;
use crate::domain::entities::Paginated;
use crate::domain::entities::User;
use crate::domain::repositories::ExerciseRepository;
use std::sync::Arc;

pub struct ReadExercises {
    exercise_repo: Arc<dyn ExerciseRepository>,
}

impl ReadExercises {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        page: PageRequest,
        name: Option<String>,
    ) -> Result<Paginated<Exercise>, AppError> {
        let name = name
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        let exercises = self
            .exercise_repo
            .get_exercises(ExerciseFilterFields {
                user_id: Some(current_user.id),
                name,
                pagination: Some(ExercisePaginationFields {
                    page: page.page,
                    per_page: page.per_page,
                }),
                ..Default::default()
            })
            .await?;

        Ok(exercises)
    }
}
