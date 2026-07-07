use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct StartWorkoutSessionRequestDTO {
    /// Nulos/ausentes iniciam um treino avulso, sem plano nem template.
    #[serde(default)]
    pub workout_plan_id: Option<Uuid>,
    #[serde(default)]
    pub workout_template_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FinishWorkoutSessionRequestDTO {
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddExerciseToWorkoutSessionRequestDTO {
    pub client_operation_id: Option<Uuid>,
    pub exercise_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddSetToWorkoutSessionRequestDTO {
    pub client_operation_id: Option<Uuid>,
    pub exercise_id: Uuid,
    pub set_type: SetTypeDTO,
    pub weight: f32,
    pub reps: u32,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateWorkoutSessionSetRequestDTO {
    pub set_type: SetTypeDTO,
    pub weight: f32,
    pub reps: u32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReorderWorkoutSessionExercisesRequestDTO {
    pub ordered_session_exercise_ids: Vec<Uuid>,
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
    Cancelled,
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
    pub workout_plan_id: Option<Uuid>,
    pub workout_template_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatusDTO,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentWorkoutSessionResponseDTO {
    pub id: Uuid,
    pub workout_plan_id: Option<Uuid>,
    pub workout_template: Option<CurrentWorkoutSessionTemplateDTO>,
    pub started_at: DateTime<Utc>,
    pub status: WorkoutSessionStatusDTO,
    pub exercises: Vec<CurrentWorkoutSessionExerciseDTO>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentWorkoutSessionTemplateDTO {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentWorkoutSessionExerciseDTO {
    pub id: Uuid,
    pub client_operation_id: Option<Uuid>,
    pub exercise: CurrentWorkoutSessionExerciseDetailsDTO,
    pub order: i32,
    pub sets: Vec<WorkoutSessionSetResponseDTO>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentWorkoutSessionExerciseDetailsDTO {
    pub id: Uuid,
    pub name: String,
    pub exercise_type: crate::adapters::http::dtos::ExerciseTypeDTO,
    pub equipment: crate::adapters::http::dtos::EquipmentDTO,
    pub muscle_group: crate::adapters::http::dtos::MuscleGroupDTO,
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
    pub client_operation_id: Option<Uuid>,
    pub exercise_id: Uuid,
    pub order: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionSetResponseDTO {
    pub id: Uuid,
    pub session_exercise_id: Uuid,
    pub client_operation_id: Option<Uuid>,
    pub set_type: SetTypeDTO,
    pub weight: f32,
    pub reps: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionHistoryItemDTO {
    pub id: Uuid,
    pub workout_plan_id: Option<Uuid>,
    pub workout_template: Option<CurrentWorkoutSessionTemplateDTO>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatusDTO,
    pub total_sets: i64,
    pub total_volume_kg: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionHistoryResponseDTO {
    pub items: Vec<WorkoutSessionHistoryItemDTO>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionWeeklySummaryResponseDTO {
    pub days: Vec<WorkoutSessionWeeklySummaryDayDTO>,
    pub total_volume_kg: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WorkoutSessionWeeklySummaryDayDTO {
    pub date: NaiveDate,
    pub day_of_week: String,
    pub trained: bool,
    pub session_id: Option<Uuid>,
}
