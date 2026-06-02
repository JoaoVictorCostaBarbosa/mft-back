use crate::domain::{
    commands::exercise_commands::ExerciseFilterFields,
    entities::{exercise::Exercise, user::User},
    enums::{equipment::Equipment, exercise_type::ExerciseType, muscle_group::MuscleGroup},
    errors::{domain_error::DomainError, exercise_error::ExerciseError},
    repositories::exercise_repository::ExerciseRepository,
};
use std::sync::Arc;

pub struct SearchExercises {
    pub exercise_repo: Arc<dyn ExerciseRepository>,
}

impl SearchExercises {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        equipment: Option<Equipment>,
        muscle_group: Option<MuscleGroup>,
        exercise_type: Option<ExerciseType>,
    ) -> Result<Vec<Exercise>, DomainError> {
        if equipment.is_none() && muscle_group.is_none() && exercise_type.is_none() {
            return Err(ExerciseError::InvalidFieldsCriteria.into());
        }

        let exercises = self
            .exercise_repo
            .get_exercises(ExerciseFilterFields {
                user_id: Some(current_user.id),
                equipment,
                muscle_group,
                exercise_type,
                ..Default::default()
            })
            .await?;

        Ok(exercises)
    }
}
