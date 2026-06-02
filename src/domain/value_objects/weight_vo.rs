use thiserror::Error;

const MIN_WEIGHT: f32 = 0.0;

#[derive(Debug, Clone, PartialEq)]
pub struct Weight(f32);

#[derive(Debug, Error)]
pub enum WeightError {
    #[error("body weight must be at least {min} kg; received {received} kg")]
    BelowMinimum { min: f32, received: f32 },
}

impl Weight {
    pub fn new(value: impl Into<f32>) -> Result<Self, WeightError> {
        let value = (value.into() * 100.0).round() / 100.0;

        if value < MIN_WEIGHT {
            return Err(WeightError::BelowMinimum {
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
