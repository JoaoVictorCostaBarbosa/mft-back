use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct WorkoutPlanRequestDTO {
    pub name: String,
    pub routine_mode: RoutineModeDTO,
}

#[derive(Deserialize, ToSchema)]
pub struct WorkoutPlanUpdateNameRequestDTO {
    pub workout_plan_id: Uuid,
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct AddWorkoutTemplateToPlanRequestDTO {
    pub day_of_week: Option<DayOfWeekDTO>,
    pub position: Option<u32>,
}

#[derive(Deserialize, ToSchema)]
pub struct AddRoutineItemToPlanRequestDTO {
    pub item_type: RoutineItemTypeDTO,
    pub workout_template_id: Option<Uuid>,
    pub day_of_week: Option<DayOfWeekDTO>,
    pub position: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DayOfWeekDTO {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RoutineModeDTO {
    Weekly,
    Sequential,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RoutineItemTypeDTO {
    Workout,
    Rest,
}

#[derive(Serialize, ToSchema)]
pub struct WorkoutPlanResponseDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub routine_mode: RoutineModeDTO,
    pub routine_items: Vec<WorkoutPlanRoutineItemResponseDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct WorkoutPlanSummaryResponseDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub routine_mode: RoutineModeDTO,
}

#[derive(Serialize, ToSchema)]
pub struct WorkoutPlanRoutineItemResponseDTO {
    pub id: Uuid,
    pub item_type: RoutineItemTypeDTO,
    pub workout_template: Option<WorkoutPlanRoutineItemTemplateResponseDTO>,
    pub day_of_week: Option<DayOfWeekDTO>,
    pub position: Option<u32>,
}

#[derive(Serialize, ToSchema)]
pub struct WorkoutPlanRoutineItemTemplateResponseDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}
