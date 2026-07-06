use crate::domain::entities::WorkoutSessionSet;
use crate::domain::enums::Equipment;
use crate::domain::enums::ExerciseType;
use crate::domain::enums::MuscleGroup;
use crate::domain::enums::WorkoutSessionStatus;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

/// Projeções de consulta (read models) das sessões de treino: shape de
/// resposta, sem invariantes — por isso vivem na application, não no domínio.
pub struct CurrentWorkoutSession {
    pub id: Uuid,
    pub workout_plan_id: Uuid,
    pub workout_template_id: Uuid,
    pub workout_template_name: String,
    pub started_at: DateTime<Utc>,
    pub status: WorkoutSessionStatus,
    pub exercises: Vec<WorkoutSessionDetailedExercise>,
}

pub struct WorkoutSessionDetailedExercise {
    pub id: Uuid,
    pub client_operation_id: Option<Uuid>,
    pub exercise: WorkoutSessionExerciseDetails,
    pub order: i32,
    pub sets: Vec<WorkoutSessionSet>,
}

pub struct WorkoutSessionExerciseDetails {
    pub id: Uuid,
    pub name: String,
    pub exercise_type: ExerciseType,
    pub equipment: Equipment,
    pub muscle_group: MuscleGroup,
}

pub struct WorkoutSessionHistoryItem {
    pub id: Uuid,
    pub workout_plan_id: Uuid,
    pub workout_template_id: Uuid,
    pub workout_template_name: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatus,
    pub total_sets: i64,
    /// Soma de peso × reps das séries da sessão, em kg.
    pub total_volume_kg: f64,
}

pub struct WorkoutSessionWeeklySummaryDay {
    pub date: NaiveDate,
    pub day_of_week: String,
    pub trained: bool,
    pub session_id: Option<Uuid>,
}

pub struct WorkoutSessionWeeklySummary {
    pub days: Vec<WorkoutSessionWeeklySummaryDay>,
    /// Soma de peso × reps de todas as séries do período, em kg.
    pub total_volume_kg: f64,
}
