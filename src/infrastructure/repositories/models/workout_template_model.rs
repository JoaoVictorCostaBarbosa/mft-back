use crate::{
    domain::{
        entities::{
            exercise::Exercise,
            workout_template::{WorkoutTemplate, WorkoutTemplateSummary},
        },
        errors::workout_template_error::WorkoutTemplateError,
        value_objects::name_vo::Name,
    },
    infrastructure::repositories::models::exercise_model::ExerciseModel,
};
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct WorkoutTemplateRowModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
pub struct WorkoutTemplateModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub exercises: Vec<ExerciseModel>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl WorkoutTemplateModel {
    pub fn new(wtr: WorkoutTemplateRowModel, exercises: Vec<ExerciseModel>) -> Self {
        Self {
            id: wtr.id,
            user_id: wtr.user_id,
            name: wtr.name,
            exercises,
            created_at: wtr.created_at,
            updated_at: wtr.updated_at,
            deleted_at: wtr.deleted_at,
        }
    }
}

impl From<&WorkoutTemplate> for WorkoutTemplateModel {
    fn from(value: &WorkoutTemplate) -> Self {
        let exercises: Vec<ExerciseModel> = value.exercises.iter().map(|e| e.into()).collect();

        Self {
            id: value.id,
            user_id: value.user_id,
            name: value.name.value().to_owned(),
            exercises: exercises,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        }
    }
}

impl TryFrom<WorkoutTemplateModel> for WorkoutTemplate {
    type Error = WorkoutTemplateError;

    fn try_from(value: WorkoutTemplateModel) -> Result<Self, Self::Error> {
        let exercises: Vec<Exercise> = value
            .exercises
            .into_iter()
            .map(|e| e.try_into())
            .collect::<Result<_, _>>()?;

        Ok(WorkoutTemplate {
            id: value.id,
            user_id: value.user_id,
            name: Name::new(value.name)?,
            exercises: exercises,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        })
    }
}

impl TryFrom<WorkoutTemplateRowModel> for WorkoutTemplateSummary {
    type Error = WorkoutTemplateError;

    fn try_from(value: WorkoutTemplateRowModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            user_id: value.user_id,
            name: Name::new(value.name)?,
        })
    }
}
