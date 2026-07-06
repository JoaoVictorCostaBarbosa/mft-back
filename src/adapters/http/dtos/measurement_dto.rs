use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, ToSchema, Deserialize)]
pub struct CreateMeasurementDTO {
    pub weight: Option<f32>,
    pub height: Option<f32>,
    pub left_calf: Option<f32>,
    pub right_calf: Option<f32>,
    pub left_quadriceps: Option<f32>,
    pub right_quadriceps: Option<f32>,
    pub hip: Option<f32>,
    pub waist: Option<f32>,
    pub chest: Option<f32>,
    pub shoulders: Option<f32>,
    pub left_arm: Option<f32>,
    pub right_arm: Option<f32>,
    pub left_forearm: Option<f32>,
    pub right_forearm: Option<f32>,
}

#[derive(Debug, ToSchema, Serialize)]
pub struct MeasurementResponse {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub weight: Option<f32>,
    pub height: Option<f32>,
    pub left_calf: Option<f32>,
    pub right_calf: Option<f32>,
    pub left_quadriceps: Option<f32>,
    pub right_quadriceps: Option<f32>,
    pub hip: Option<f32>,
    pub waist: Option<f32>,
    pub chest: Option<f32>,
    pub shoulders: Option<f32>,
    pub left_arm: Option<f32>,
    pub right_arm: Option<f32>,
    pub left_forearm: Option<f32>,
    pub right_forearm: Option<f32>,
}
