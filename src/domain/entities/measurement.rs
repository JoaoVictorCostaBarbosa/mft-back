use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::errors::MeasurementError;
use crate::domain::value_objects::BodyHeight;
use crate::domain::value_objects::BodyPartMeasure;
use crate::domain::value_objects::BodyWeight;

#[derive(Debug, Clone)]
pub struct Measurement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub weight: Option<BodyWeight>,
    pub height: Option<BodyHeight>,
    pub left_calf: Option<BodyPartMeasure>,
    pub right_calf: Option<BodyPartMeasure>,
    pub left_quadriceps: Option<BodyPartMeasure>,
    pub right_quadriceps: Option<BodyPartMeasure>,
    pub hip: Option<BodyPartMeasure>,
    pub waist: Option<BodyPartMeasure>,
    pub chest: Option<BodyPartMeasure>,
    pub shoulders: Option<BodyPartMeasure>,
    pub left_arm: Option<BodyPartMeasure>,
    pub right_arm: Option<BodyPartMeasure>,
    pub left_forearm: Option<BodyPartMeasure>,
    pub right_forearm: Option<BodyPartMeasure>,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Measurement {
    pub fn new(
        user_id: Uuid,
        weight: Option<f32>,
        height: Option<f32>,
        left_calf: Option<f32>,
        right_calf: Option<f32>,
        left_quadriceps: Option<f32>,
        right_quadriceps: Option<f32>,
        hip: Option<f32>,
        waist: Option<f32>,
        chest: Option<f32>,
        shoulders: Option<f32>,
        left_arm: Option<f32>,
        right_arm: Option<f32>,
        left_forearm: Option<f32>,
        right_forearm: Option<f32>,
    ) -> Result<Self, MeasurementError> {
        let now: DateTime<Utc> = Utc::now();

        let weight = weight.map(BodyWeight::new).transpose()?;
        let height = height.map(BodyHeight::new).transpose()?;
        let left_calf = left_calf.map(BodyPartMeasure::new).transpose()?;
        let right_calf = right_calf.map(BodyPartMeasure::new).transpose()?;
        let left_quadriceps = left_quadriceps.map(BodyPartMeasure::new).transpose()?;
        let right_quadriceps = right_quadriceps.map(BodyPartMeasure::new).transpose()?;
        let hip = hip.map(BodyPartMeasure::new).transpose()?;
        let waist = waist.map(BodyPartMeasure::new).transpose()?;
        let chest = chest.map(BodyPartMeasure::new).transpose()?;
        let shoulders = shoulders.map(BodyPartMeasure::new).transpose()?;
        let left_arm = left_arm.map(BodyPartMeasure::new).transpose()?;
        let right_arm = right_arm.map(BodyPartMeasure::new).transpose()?;
        let left_forearm = left_forearm.map(BodyPartMeasure::new).transpose()?;
        let right_forearm = right_forearm.map(BodyPartMeasure::new).transpose()?;

        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            weight,
            height,
            left_calf,
            right_calf,
            left_quadriceps,
            right_quadriceps,
            hip,
            waist,
            chest,
            shoulders,
            left_arm,
            right_arm,
            left_forearm,
            right_forearm,
            created_at: now,
            deleted_at: None,
        })
    }
}
