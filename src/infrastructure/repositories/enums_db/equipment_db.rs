use crate::domain::enums::Equipment;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "equipment_enum", rename_all = "snake_case")]
pub enum EquipmentDb {
    Barbell,
    Dumbbell,
    Machine,
    Bodyweight,
    Kettlebell,
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
            EquipmentDb::Kettlebell => Equipment::Kettlebell,
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
            Equipment::Kettlebell => EquipmentDb::Kettlebell,
            Equipment::ResistanceBand => EquipmentDb::ResistanceBand,
            Equipment::Other => EquipmentDb::Other,
        }
    }
}
