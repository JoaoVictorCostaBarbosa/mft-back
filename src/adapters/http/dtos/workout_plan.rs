use crate::adapters::http::dtos::workout_template::WorkoutTemplateSummaryResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct WorkoutPlanRequestDTO {
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct WorkoutPlanUpdateNameRequestDTO {
    pub workout_plan_id: Uuid,
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct WorkoutPlanResponseDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub templates: Vec<WorkoutTemplateSummaryResponse>,
}

#[derive(Serialize, ToSchema)]
pub struct WorkoutPlanSummaryResponseDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}
