use crate::domain::enums::SetType;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct FinishWorkoutSessionInput {
    pub session_id: Uuid,
    pub finished_at: Option<DateTime<Utc>>,
}

pub struct AddExerciseToSessionInput {
    pub session_id: Uuid,
    pub exercise_id: Uuid,
    pub client_operation_id: Option<Uuid>,
}

pub struct AddSetToSessionInput {
    pub session_id: Uuid,
    pub exercise_id: Uuid,
    pub set_type: SetType,
    pub weight: f32,
    pub reps: u32,
    pub client_operation_id: Option<Uuid>,
    pub completed_at: Option<DateTime<Utc>>,
}

pub struct UpdateSessionSetInput {
    pub session_id: Uuid,
    pub set_id: Uuid,
    pub set_type: SetType,
    pub weight: f32,
    pub reps: u32,
}
