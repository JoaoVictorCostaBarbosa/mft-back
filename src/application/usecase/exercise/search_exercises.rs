use crate::application::dtos::exercise::SearchExercisesInput;
use crate::application::errors::AppError;
use crate::domain::commands::ExerciseFilterFields;
use crate::domain::commands::ExercisePaginationFields;
use crate::domain::entities::Exercise;
use crate::domain::entities::Paginated;
use crate::domain::entities::User;
use crate::domain::errors::ExerciseError;
use crate::domain::repositories::ExerciseRepository;
use std::sync::Arc;

pub struct SearchExercises {
    exercise_repo: Arc<dyn ExerciseRepository>,
}

impl SearchExercises {
    pub fn new(exercise_repo: Arc<dyn ExerciseRepository>) -> Self {
        Self { exercise_repo }
    }

    pub async fn execute(
        &self,
        current_user: User,
        input: SearchExercisesInput,
    ) -> Result<Paginated<Exercise>, AppError> {
        if input.equipment.is_none()
            && input.muscle_group.is_none()
            && input.exercise_type.is_none()
        {
            return Err(ExerciseError::InvalidFieldsCriteria.into());
        }

        let exercises = self
            .exercise_repo
            .get_exercises(ExerciseFilterFields {
                user_id: Some(current_user.id),
                equipment: input.equipment,
                muscle_group: input.muscle_group,
                exercise_type: input.exercise_type,
                pagination: Some(ExercisePaginationFields {
                    page: input.page.page,
                    per_page: input.page.per_page,
                }),
                ..Default::default()
            })
            .await?;

        Ok(exercises)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::pagination::PageRequest;
    use crate::domain::entities::Exercise;
    use crate::domain::enums::{Equipment, ExerciseType, MuscleGroup};
    use crate::domain::errors::DomainError;
    use crate::test_support::fakes::InMemoryExerciseRepository;
    use crate::test_support::fixtures;

    fn exercise(name: &str, equipment: Equipment) -> Exercise {
        Exercise::new(
            None,
            name.to_string(),
            ExerciseType::Strength,
            equipment,
            MuscleGroup::Chest,
        )
        .unwrap()
    }

    fn input(equipment: Option<Equipment>) -> SearchExercisesInput {
        SearchExercisesInput {
            equipment,
            muscle_group: None,
            exercise_type: None,
            page: PageRequest::new(None, None),
        }
    }

    #[tokio::test]
    async fn rejects_search_without_criteria() {
        let repo = Arc::new(InMemoryExerciseRepository::default());
        let use_case = SearchExercises::new(repo);

        let err = match use_case.execute(fixtures::user(), input(None)).await {
            Ok(_) => panic!("expected error"),
            Err(err) => err,
        };

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Exercise(ExerciseError::InvalidFieldsCriteria))
        ));
    }

    #[tokio::test]
    async fn filters_by_equipment() {
        let repo = Arc::new(InMemoryExerciseRepository::with_exercises(vec![
            exercise("Supino", Equipment::Barbell),
            exercise("Crucifixo", Equipment::Dumbbell),
        ]));
        let use_case = SearchExercises::new(repo);

        let result = use_case
            .execute(fixtures::user(), input(Some(Equipment::Barbell)))
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].equipment, Equipment::Barbell);
    }
}
