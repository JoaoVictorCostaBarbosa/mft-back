use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::adapters::http::dtos::exercise_dto::ExerciseResponseDTO;

#[derive(Debug, Deserialize, ToSchema)]
pub struct WorkoutTemplateRequestDTO {
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct WorkoutTemplateExerciseDTO {
    pub id: Uuid,
    pub exercise_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct WorkoutTemplateUpdateNameDTO {
    pub workout_id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutTemplateResponseDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub exercises: Vec<ExerciseResponseDTO>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutTemplateSummaryResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}
