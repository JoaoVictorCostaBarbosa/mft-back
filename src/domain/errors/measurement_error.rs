use crate::domain::value_objects::{
    body_height_vo::BodyHeightError, body_part_vo::BodyPartMeasureError,
    body_weight_vo::BodyWeightError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MeasurementError {
    #[error("weight error: {0}")]
    Weight(#[from] BodyWeightError),
    #[error("height error: {0}")]
    Height(#[from] BodyHeightError),
    #[error("part measurement error: {0}")]
    PartMeasurement(#[from] BodyPartMeasureError),
}
