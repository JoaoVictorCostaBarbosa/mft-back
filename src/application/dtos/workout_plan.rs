use crate::domain::enums::DayOfWeek;
use crate::domain::enums::RoutineItemType;
use crate::domain::enums::RoutineMode;
use uuid::Uuid;

pub struct WorkoutPlanRequest {
    pub name: String,
    pub routine_mode: RoutineMode,
}

pub struct WorkoutPlanUpdateRequest {
    pub id: Uuid,
    pub name: Option<String>,
}

pub struct AddRoutineItemInput {
    pub workout_plan_id: Uuid,
    pub workout_template_id: Option<Uuid>,
    pub item_type: RoutineItemType,
    pub day_of_week: Option<DayOfWeek>,
    pub position: Option<u32>,
}

pub struct UpdateRoutineItemInput {
    pub workout_plan_id: Uuid,
    pub routine_item_id: Uuid,
    pub item_type: Option<RoutineItemType>,
    pub workout_template_id: Option<Uuid>,
    pub day_of_week: Option<DayOfWeek>,
    pub position: Option<u32>,
}
