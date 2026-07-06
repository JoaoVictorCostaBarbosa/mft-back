use crate::domain::enums::ExerciseType;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ExerciseTypeDTO {
    Strength,
    Cardio,
    Flexibility,
    Balance,
}

impl From<ExerciseTypeDTO> for ExerciseType {
    fn from(value: ExerciseTypeDTO) -> Self {
        match value {
            ExerciseTypeDTO::Strength => ExerciseType::Strength,
            ExerciseTypeDTO::Cardio => ExerciseType::Cardio,
            ExerciseTypeDTO::Flexibility => ExerciseType::Flexibility,
            ExerciseTypeDTO::Balance => ExerciseType::Balance,
        }
    }
}

impl From<ExerciseType> for ExerciseTypeDTO {
    fn from(value: ExerciseType) -> Self {
        match value {
            ExerciseType::Strength => ExerciseTypeDTO::Strength,
            ExerciseType::Cardio => ExerciseTypeDTO::Cardio,
            ExerciseType::Flexibility => ExerciseTypeDTO::Flexibility,
            ExerciseType::Balance => ExerciseTypeDTO::Balance,
        }
    }
}
