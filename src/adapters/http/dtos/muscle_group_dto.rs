use crate::domain::enums::MuscleGroup;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum MuscleGroupDTO {
    Chest,
    Back,
    Shoulders,
    Arms,
    Legs,
    Core,
    FullBody,
    Other,
}

impl From<MuscleGroupDTO> for MuscleGroup {
    fn from(value: MuscleGroupDTO) -> Self {
        match value {
            MuscleGroupDTO::Chest => MuscleGroup::Chest,
            MuscleGroupDTO::Back => MuscleGroup::Back,
            MuscleGroupDTO::Shoulders => MuscleGroup::Shoulders,
            MuscleGroupDTO::Arms => MuscleGroup::Arms,
            MuscleGroupDTO::Legs => MuscleGroup::Legs,
            MuscleGroupDTO::Core => MuscleGroup::Core,
            MuscleGroupDTO::FullBody => MuscleGroup::FullBody,
            MuscleGroupDTO::Other => MuscleGroup::Other,
        }
    }
}

impl From<MuscleGroup> for MuscleGroupDTO {
    fn from(value: MuscleGroup) -> Self {
        match value {
            MuscleGroup::Chest => MuscleGroupDTO::Chest,
            MuscleGroup::Back => MuscleGroupDTO::Back,
            MuscleGroup::Shoulders => MuscleGroupDTO::Shoulders,
            MuscleGroup::Arms => MuscleGroupDTO::Arms,
            MuscleGroup::Legs => MuscleGroupDTO::Legs,
            MuscleGroup::Core => MuscleGroupDTO::Core,
            MuscleGroup::FullBody => MuscleGroupDTO::FullBody,
            MuscleGroup::Other => MuscleGroupDTO::Other,
        }
    }
}
