use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct StartWorkoutSessionRequestDTO {
    pub workout_plan_id: Uuid,
    pub workout_template_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FinishWorkoutSessionRequestDTO {
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddExerciseToWorkoutSessionRequestDTO {
    pub exercise_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddSetToWorkoutSessionRequestDTO {
    pub exercise_id: Uuid,
    pub set_type: SetTypeDTO,
    pub weight: f32,
    pub reps: u32,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct WorkoutSessionWeeklySummaryQueryDTO {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum WorkoutSessionStatusDTO {
    InProgress,
    Finished,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SetTypeDTO {
    Warmup,
    Working,
    Drop,
    Failure,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionResponseDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workout_plan_id: Uuid,
    pub workout_template_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatusDTO,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentWorkoutSessionResponseDTO {
    pub id: Uuid,
    pub workout_template: CurrentWorkoutSessionTemplateDTO,
    pub started_at: DateTime<Utc>,
    pub status: WorkoutSessionStatusDTO,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentWorkoutSessionTemplateDTO {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FinishedWorkoutSessionResponseDTO {
    pub id: Uuid,
    pub status: WorkoutSessionStatusDTO,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionExerciseResponseDTO {
    pub id: Uuid,
    pub workout_session_id: Uuid,
    pub exercise_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionSetResponseDTO {
    pub id: Uuid,
    pub session_exercise_id: Uuid,
    pub set_type: SetTypeDTO,
    pub weight: f32,
    pub reps: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionHistoryItemDTO {
    pub id: Uuid,
    pub workout_plan_id: Uuid,
    pub workout_template: CurrentWorkoutSessionTemplateDTO,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatusDTO,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionHistoryResponseDTO {
    pub items: Vec<WorkoutSessionHistoryItemDTO>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionWeeklySummaryResponseDTO {
    pub days: Vec<WorkoutSessionWeeklySummaryDayDTO>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionWeeklySummaryDayDTO {
    pub date: NaiveDate,
    pub day_of_week: String,
    pub trained: bool,
    pub session_id: Option<Uuid>,
}
