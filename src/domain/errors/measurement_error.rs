use crate::domain::value_objects::BodyHeightError;
use crate::domain::value_objects::BodyPartMeasureError;
use crate::domain::value_objects::BodyWeightError;
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
