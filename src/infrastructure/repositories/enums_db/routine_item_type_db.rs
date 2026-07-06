use crate::domain::enums::RoutineItemType;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "routine_item_type_enum", rename_all = "snake_case")]
pub enum RoutineItemTypeDb {
    Workout,
    Rest,
}

impl From<RoutineItemTypeDb> for RoutineItemType {
    fn from(value: RoutineItemTypeDb) -> Self {
        match value {
            RoutineItemTypeDb::Workout => RoutineItemType::Workout,
            RoutineItemTypeDb::Rest => RoutineItemType::Rest,
        }
    }
}

impl From<RoutineItemType> for RoutineItemTypeDb {
    fn from(value: RoutineItemType) -> Self {
        match value {
            RoutineItemType::Workout => RoutineItemTypeDb::Workout,
            RoutineItemType::Rest => RoutineItemTypeDb::Rest,
        }
    }
}
