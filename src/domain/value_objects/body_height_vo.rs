use thiserror::Error;

const MIN_HEIGHT: f32 = 100.0;
const MAX_HEIGHT: f32 = 272.0;

#[derive(Debug, Clone, PartialEq)]
pub struct BodyHeight(f32);

#[derive(Debug, Error)]
pub enum BodyHeightError {
    #[error("height too small: {0}")]
    TooSmall(f32),

    #[error("height too large {0}")]
    TooLarge(f32),
}

impl BodyHeight {
    pub fn new(value: impl Into<f32>) -> Result<Self, BodyHeightError> {
        let value = (value.into() * 100.0).round() / 100.0;

        if value < MIN_HEIGHT {
            return Err(BodyHeightError::TooSmall(value));
        }

        if value > MAX_HEIGHT {
            return Err(BodyHeightError::TooLarge(value));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}
