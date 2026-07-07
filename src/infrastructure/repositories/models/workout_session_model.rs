use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::infrastructure::repositories::enums_db::EquipmentDb;
use crate::infrastructure::repositories::enums_db::ExerciseTypeDb;
use crate::infrastructure::repositories::enums_db::MuscleGroupDb;
use crate::infrastructure::repositories::enums_db::SetTypeDb;
use crate::infrastructure::repositories::enums_db::WorkoutSessionStatusDb;

#[derive(Debug, FromRow)]
pub struct WorkoutSessionRowModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workout_plan_id: Option<Uuid>,
    pub workout_template_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatusDb,
}

#[derive(Debug, FromRow)]
pub struct CurrentWorkoutSessionRowModel {
    pub id: Uuid,
    pub workout_plan_id: Option<Uuid>,
    pub workout_template_id: Option<Uuid>,
    pub workout_template_name: Option<String>,
    pub started_at: DateTime<Utc>,
    pub status: WorkoutSessionStatusDb,
}

#[derive(Debug, FromRow)]
pub struct WorkoutSessionHistoryRowModel {
    pub id: Uuid,
    pub workout_plan_id: Option<Uuid>,
    pub workout_template_id: Option<Uuid>,
    pub workout_template_name: Option<String>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatusDb,
    pub total_sets: i64,
    pub total_volume_kg: f64,
}

#[derive(Debug, FromRow)]
pub struct WorkoutSessionExerciseRowModel {
    pub id: Uuid,
    pub workout_session_id: Uuid,
    pub client_operation_id: Option<Uuid>,
    pub exercise_id: Uuid,
    pub order: i32,
}

#[derive(Debug, FromRow)]
pub struct WorkoutSessionSetRowModel {
    pub id: Uuid,
    pub session_exercise_id: Uuid,
    pub client_operation_id: Option<Uuid>,
    pub set_type: SetTypeDb,
    pub weight: f32,
    pub reps: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct WorkoutSessionWeeklySummaryRowModel {
    pub date: NaiveDate,
    pub day_of_week: String,
    pub session_id: Option<Uuid>,
}

#[derive(Debug, FromRow)]
pub struct WorkoutSessionDetailedExerciseRowModel {
    pub id: Uuid,
    pub client_operation_id: Option<Uuid>,
    pub exercise_id: Uuid,
    pub exercise_name: String,
    pub exercise_type: ExerciseTypeDb,
    pub equipment: EquipmentDb,
    pub muscle_group: MuscleGroupDb,
    pub order: i32,
}
