use crate::domain::value_objects::weight_vo::WeightError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SetLogError {
    #[error("weight error: {0}")]
    Weight(#[from] WeightError),

    #[error("set log lot found")]
    NotFound,
}
