use crate::domain::enums::routine_mode::RoutineMode;
use uuid::Uuid;

pub struct WorkoutPlanRequest {
    pub name: String,
    pub routine_mode: RoutineMode,
}

pub struct WorkoutPlanUpdateRequest {
    pub id: Uuid,
    pub name: Option<String>,
}
