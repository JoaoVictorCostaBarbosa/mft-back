use crate::adapters::http::dtos::{
    equipment_dto::EquipmentDTO, exercise_type_dto::ExerciseTypeDTO,
    muscle_group_dto::MuscleGroupDTO,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExerciseRequest {
    pub name: String,
    pub exercise_type: ExerciseTypeDTO,
    pub equipment: EquipmentDTO,
    pub muscle_group: MuscleGroupDTO,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExerciseResponseDTO {
    pub id: Uuid,
    pub name: String,
    pub exercise_type: ExerciseTypeDTO,
    pub equipment: EquipmentDTO,
    pub muscle_group: MuscleGroupDTO,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExerciseUpdateRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub exercise_type: Option<ExerciseTypeDTO>,
    pub equipment: Option<EquipmentDTO>,
    pub muscle_group: Option<MuscleGroupDTO>,
}
