use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::infrastructure::repositories::enums_db::{
    set_type_db::SetTypeDb, workout_session_status_db::WorkoutSessionStatusDb,
};

#[derive(Debug, FromRow)]
pub struct WorkoutSessionRowModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workout_plan_id: Uuid,
    pub workout_template_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatusDb,
}

#[derive(Debug, FromRow)]
pub struct CurrentWorkoutSessionRowModel {
    pub id: Uuid,
    pub workout_template_id: Uuid,
    pub workout_template_name: String,
    pub started_at: DateTime<Utc>,
    pub status: WorkoutSessionStatusDb,
}

#[derive(Debug, FromRow)]
pub struct WorkoutSessionHistoryRowModel {
    pub id: Uuid,
    pub workout_plan_id: Uuid,
    pub workout_template_id: Uuid,
    pub workout_template_name: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatusDb,
}

#[derive(Debug, FromRow)]
pub struct WorkoutSessionExerciseRowModel {
    pub id: Uuid,
    pub workout_session_id: Uuid,
    pub exercise_id: Uuid,
}

#[derive(Debug, FromRow)]
pub struct WorkoutSessionSetRowModel {
    pub id: Uuid,
    pub session_exercise_id: Uuid,
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
