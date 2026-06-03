use crate::domain::enums::{
    equipment::Equipment, exercise_type::ExerciseType, muscle_group::MuscleGroup,
};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct ExerciseUpdateFields {
    pub id: Uuid,
    pub name: Option<String>,
    pub exercise_type: Option<ExerciseType>,
    pub equipment: Option<Equipment>,
    pub muscle_group: Option<MuscleGroup>,
}

#[derive(Debug, Default)]
pub struct ExerciseFilterFields {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub equipment: Option<Equipment>,
    pub exercise_type: Option<ExerciseType>,
    pub muscle_group: Option<MuscleGroup>,
    pub pagination: Option<ExercisePaginationFields>,
}

#[derive(Debug, Clone, Copy)]
pub struct ExercisePaginationFields {
    pub page: u32,
    pub per_page: u32,
}

impl ExercisePaginationFields {
    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }

    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page) as i64
    }
}

impl ExerciseUpdateFields {
    pub fn is_empty(&self) -> bool {
        if self.name.is_none()
            && self.equipment.is_none()
            && self.muscle_group.is_none()
            && self.exercise_type.is_none()
        {
            return true;
        }
        false
    }
}
