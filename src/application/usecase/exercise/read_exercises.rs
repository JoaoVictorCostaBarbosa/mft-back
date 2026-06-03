use crate::domain::{
    commands::exercise_commands::{ExerciseFilterFields, ExercisePaginationFields},
    entities::{exercise::Exercise, pagination::Paginated, user::User},
    errors::domain_error::DomainError,
    repositories::exercise_repository::ExerciseRepository,
};
use std::sync::Arc;

pub struct ReadExercises {
    pub exercise_repo: Arc<dyn ExerciseRepository>,
}

impl ReadExercises {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        pagination: ExercisePaginationFields,
    ) -> Result<Paginated<Exercise>, DomainError> {
        let exercises = self
            .exercise_repo
            .get_exercises(ExerciseFilterFields {
                user_id: Some(current_user.id),
                pagination: Some(pagination),
                ..Default::default()
            })
            .await?;

        Ok(exercises)
    }
}
