use crate::domain::enums::workout_session_status::WorkoutSessionStatus;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "workout_session_status_enum", rename_all = "snake_case")]
pub enum WorkoutSessionStatusDb {
    InProgress,
    Finished,
    Cancelled,
}

impl From<WorkoutSessionStatusDb> for WorkoutSessionStatus {
    fn from(value: WorkoutSessionStatusDb) -> Self {
        match value {
            WorkoutSessionStatusDb::InProgress => WorkoutSessionStatus::InProgress,
            WorkoutSessionStatusDb::Finished => WorkoutSessionStatus::Finished,
            WorkoutSessionStatusDb::Cancelled => WorkoutSessionStatus::Cancelled,
        }
    }
}

impl From<WorkoutSessionStatus> for WorkoutSessionStatusDb {
    fn from(value: WorkoutSessionStatus) -> Self {
        match value {
            WorkoutSessionStatus::InProgress => WorkoutSessionStatusDb::InProgress,
            WorkoutSessionStatus::Finished => WorkoutSessionStatusDb::Finished,
            WorkoutSessionStatus::Cancelled => WorkoutSessionStatusDb::Cancelled,
        }
    }
}
