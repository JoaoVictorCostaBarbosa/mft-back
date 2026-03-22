use crate::domain::{
    enums::{equipment::Equipment, exercise_type::ExerciseType, muscle_group::MuscleGroup},
    errors::exercise_error::ExerciseError,
    value_objects::name_vo::Name,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Exercise {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: Name,
    pub exercise_type: ExerciseType,
    pub equipment: Equipment,
    pub muscle_group: MuscleGroup,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Exercise {
    pub fn new(
        user_id: Option<Uuid>,
        name: String,
        exercise_type: ExerciseType,
        equipment: Equipment,
        muscle_group: MuscleGroup,
    ) -> Result<Self, ExerciseError> {
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            name: Name::new(name)?,
            exercise_type,
            equipment,
            muscle_group,
            created_at: Utc::now(),
            deleted_at: None,
        })
    }
}
