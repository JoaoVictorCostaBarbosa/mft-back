use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::entities::Measurement;
use crate::domain::errors::MeasurementError;
use crate::domain::value_objects::BodyHeight;
use crate::domain::value_objects::BodyPartMeasure;
use crate::domain::value_objects::BodyWeight;

#[derive(Debug, FromRow)]
pub struct MeasurementModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub left_calf: Option<f64>,
    pub right_calf: Option<f64>,
    pub left_quadriceps: Option<f64>,
    pub right_quadriceps: Option<f64>,
    pub hip: Option<f64>,
    pub waist: Option<f64>,
    pub chest: Option<f64>,
    pub shoulders: Option<f64>,
    pub left_arm: Option<f64>,
    pub right_arm: Option<f64>,
    pub left_forearm: Option<f64>,
    pub right_forearm: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl MeasurementModel {
    pub fn to_domain(&self) -> Result<Measurement, MeasurementError> {
        Ok(Measurement {
            id: self.id,
            user_id: self.user_id,

            weight: map_optional_vo(self.weight, BodyWeight::new)?,
            height: map_optional_vo(self.height, BodyHeight::new)?,

            left_calf: map_optional_vo(self.left_calf, BodyPartMeasure::new)?,
            right_calf: map_optional_vo(self.right_calf, BodyPartMeasure::new)?,
            left_quadriceps: map_optional_vo(self.left_quadriceps, BodyPartMeasure::new)?,
            right_quadriceps: map_optional_vo(self.right_quadriceps, BodyPartMeasure::new)?,
            hip: map_optional_vo(self.hip, BodyPartMeasure::new)?,
            waist: map_optional_vo(self.waist, BodyPartMeasure::new)?,
            chest: map_optional_vo(self.chest, BodyPartMeasure::new)?,
            shoulders: map_optional_vo(self.shoulders, BodyPartMeasure::new)?,
            left_arm: map_optional_vo(self.left_arm, BodyPartMeasure::new)?,
            right_arm: map_optional_vo(self.right_arm, BodyPartMeasure::new)?,
            left_forearm: map_optional_vo(self.left_forearm, BodyPartMeasure::new)?,
            right_forearm: map_optional_vo(self.right_forearm, BodyPartMeasure::new)?,

            created_at: self.created_at,
            deleted_at: self.deleted_at,
        })
    }
}

fn map_optional_vo<T, E>(
    value: Option<f64>,
    constructor: impl Fn(f32) -> Result<T, E>,
) -> Result<Option<T>, E> {
    value.map(|v| constructor(v as f32)).transpose()
}
