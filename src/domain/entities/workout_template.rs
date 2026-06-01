use crate::domain::{
    entities::{exercise::Exercise, user::User},
    errors::{exercise_error::ExerciseError, workout_template_error::WorkoutTemplateError},
    value_objects::name_vo::Name,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct WorkoutTemplate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Name,
    pub exercises: Vec<Exercise>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(PartialEq)]
pub struct WorkoutTemplateSummary {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Name,
}

impl WorkoutTemplate {
    pub fn new(
        user_id: Uuid,
        name: String,
        exercises: Vec<Exercise>,
    ) -> Result<Self, WorkoutTemplateError> {
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            name: Name::new(name)?,
            exercises,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        })
    }

    pub fn add_exercise(&mut self, exercise: Exercise) -> Result<(), WorkoutTemplateError> {
        if self.exercises.contains(&exercise) {
            return Err(WorkoutTemplateError::AlreadyAdded);
        }

        self.exercises.push(exercise);

        Ok(())
    }

    pub fn remove_exercise(&mut self, exercise_id: Uuid) -> Result<(), WorkoutTemplateError> {
        let index = self
            .exercises
            .iter()
            .position(|e| e.id == exercise_id)
            .ok_or(ExerciseError::NotFound)?;

        self.exercises.remove(index);

        Ok(())
    }

    pub fn assert_owner(&self, user: &User) -> Result<(), WorkoutTemplateError> {
        if self.user_id != user.id {
            return Err(WorkoutTemplateError::Forbidden);
        }
        Ok(())
    }

    pub fn update_template(&mut self, name: Option<String>) -> Result<(), WorkoutTemplateError> {
        let mut changed = false;

        if let Some(n) = name {
            self.name = Name::new(n)?;
            changed = true;
        }

        if changed {
            self.updated_at = Some(Utc::now());
        }

        Ok(())
    }
}

impl WorkoutTemplateSummary {
    pub fn new(id: Uuid, user_id: Uuid, name: Name) -> Self {
        Self { id, user_id, name }
    }
}
