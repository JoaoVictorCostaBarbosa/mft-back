use crate::domain::enums::RoutineMode;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "routine_mode_enum", rename_all = "snake_case")]
pub enum RoutineModeDb {
    Weekly,
    Sequential,
}

impl From<RoutineModeDb> for RoutineMode {
    fn from(value: RoutineModeDb) -> Self {
        match value {
            RoutineModeDb::Weekly => RoutineMode::Weekly,
            RoutineModeDb::Sequential => RoutineMode::Sequential,
        }
    }
}

impl From<RoutineMode> for RoutineModeDb {
    fn from(value: RoutineMode) -> Self {
        match value {
            RoutineMode::Weekly => RoutineModeDb::Weekly,
            RoutineMode::Sequential => RoutineModeDb::Sequential,
        }
    }
}
