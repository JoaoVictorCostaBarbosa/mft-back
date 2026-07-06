use crate::domain::enums::Equipment;
use crate::domain::enums::ExerciseType;
use crate::domain::enums::MuscleGroup;
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
    pub name: Option<String>,
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
