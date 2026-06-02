use thiserror::Error;

const MIN_WEIGHT: f32 = 30.0;

#[derive(Debug, Clone, PartialEq)]
pub struct BodyWeight(f32);

#[derive(Debug, Error)]
pub enum BodyWeightError {
    #[error("body weight must be at least {min} kg; received {received} kg")]
    BelowMinimum { min: f32, received: f32 },
}

impl BodyWeight {
    pub fn new(value: impl Into<f32>) -> Result<Self, BodyWeightError> {
        let value = (value.into() * 100.0).round() / 100.0;

        if value < MIN_WEIGHT {
            return Err(BodyWeightError::BelowMinimum {
                min: MIN_WEIGHT,
                received: value,
            });
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}
