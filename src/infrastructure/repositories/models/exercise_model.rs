use crate::{
    domain::{
        entities::exercise::Exercise, errors::user_error::UserError, value_objects::name_vo::Name,
    },
    infrastructure::repositories::enums_db::{
        equipment_db::EquipmentDb, exercise_type_db::ExerciseTypeDb, muscle_group_db::MuscleGroupDb,
    },
};
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct ExerciseModel {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub exercise_type: ExerciseTypeDb,
    pub equipment: EquipmentDb,
    pub muscle_group: MuscleGroupDb,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<ExerciseModel> for Exercise {
    type Error = UserError;

    fn try_from(model: ExerciseModel) -> Result<Self, Self::Error> {
        Ok(Exercise {
            id: model.id,
            user_id: model.user_id,
            name: Name::new(model.name)?,
            exercise_type: model.exercise_type.into(),
            equipment: model.equipment.into(),
            muscle_group: model.muscle_group.into(),
            created_at: model.created_at,
            deleted_at: model.deleted_at,
        })
    }
}

impl From<&Exercise> for ExerciseModel {
    fn from(exercise: &Exercise) -> Self {
        Self {
            id: exercise.id,
            user_id: exercise.user_id,
            name: exercise.name.value().to_owned(),
            exercise_type: exercise.exercise_type.into(),
            equipment: exercise.equipment.into(),
            muscle_group: exercise.muscle_group.into(),
            created_at: exercise.created_at,
            deleted_at: exercise.deleted_at,
        }
    }
}
