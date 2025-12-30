use crate::domain::enums::equipment::Equipment;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "equipment_enum", rename_all = "lowercase")]
pub enum EquipmentDb {
    Barbell,
    Dumbbell,
    Machine,
    Bodyweight,
    Kettlerbell,
    ResistanceBand,
    Other,
}

impl From<EquipmentDb> for Equipment {
    fn from(value: EquipmentDb) -> Self {
        match value {
            EquipmentDb::Barbell => Equipment::Barbell,
            EquipmentDb::Dumbbell => Equipment::Dumbbell,
            EquipmentDb::Machine => Equipment::Machine,
            EquipmentDb::Bodyweight => Equipment::Bodyweight,
            EquipmentDb::Kettlerbell => Equipment::Kettlerbell,
            EquipmentDb::ResistanceBand => Equipment::ResistanceBand,
            EquipmentDb::Other => Equipment::Other,
        }
    }
}

impl From<Equipment> for EquipmentDb {
    fn from(value: Equipment) -> Self {
        match value {
            Equipment::Barbell => EquipmentDb::Barbell,
            Equipment::Dumbbell => EquipmentDb::Dumbbell,
            Equipment::Machine => EquipmentDb::Machine,
            Equipment::Bodyweight => EquipmentDb::Bodyweight,
            Equipment::Kettlerbell => EquipmentDb::Kettlerbell,
            Equipment::ResistanceBand => EquipmentDb::ResistanceBand,
            Equipment::Other => EquipmentDb::Other,
        }
    }
}
