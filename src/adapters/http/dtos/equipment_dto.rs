use crate::domain::enums::Equipment;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum EquipmentDTO {
    Barbell,
    Dumbbell,
    Machine,
    Bodyweight,
    #[serde(alias = "Kettlerbell")]
    Kettlebell,
    ResistanceBand,
    Other,
}

impl From<EquipmentDTO> for Equipment {
    fn from(value: EquipmentDTO) -> Self {
        match value {
            EquipmentDTO::Barbell => Equipment::Barbell,
            EquipmentDTO::Dumbbell => Equipment::Dumbbell,
            EquipmentDTO::Machine => Equipment::Machine,
            EquipmentDTO::Bodyweight => Equipment::Bodyweight,
            EquipmentDTO::Kettlebell => Equipment::Kettlebell,
            EquipmentDTO::ResistanceBand => Equipment::ResistanceBand,
            EquipmentDTO::Other => Equipment::Other,
        }
    }
}

impl From<Equipment> for EquipmentDTO {
    fn from(value: Equipment) -> Self {
        match value {
            Equipment::Barbell => EquipmentDTO::Barbell,
            Equipment::Dumbbell => EquipmentDTO::Dumbbell,
            Equipment::Machine => EquipmentDTO::Machine,
            Equipment::Bodyweight => EquipmentDTO::Bodyweight,
            Equipment::Kettlebell => EquipmentDTO::Kettlebell,
            Equipment::ResistanceBand => EquipmentDTO::ResistanceBand,
            Equipment::Other => EquipmentDTO::Other,
        }
    }
}
