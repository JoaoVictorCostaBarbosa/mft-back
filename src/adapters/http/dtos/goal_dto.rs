use crate::domain::enums::Goal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum GoalDTO {
    Muscle,
    Loss,
    Strength,
    Health,
}

impl From<Goal> for GoalDTO {
    fn from(value: Goal) -> Self {
        match value {
            Goal::Muscle => GoalDTO::Muscle,
            Goal::Loss => GoalDTO::Loss,
            Goal::Strength => GoalDTO::Strength,
            Goal::Health => GoalDTO::Health,
        }
    }
}

impl From<GoalDTO> for Goal {
    fn from(value: GoalDTO) -> Self {
        match value {
            GoalDTO::Muscle => Goal::Muscle,
            GoalDTO::Loss => Goal::Loss,
            GoalDTO::Strength => Goal::Strength,
            GoalDTO::Health => Goal::Health,
        }
    }
}
