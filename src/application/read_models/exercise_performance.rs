use crate::domain::enums::SetType;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ExerciseLastPerformance {
    pub exercise_id: Uuid,
    pub last_session_id: Uuid,
    pub performed_at: DateTime<Utc>,
    pub sets: Vec<ExerciseLastPerformanceSet>,
}

#[derive(Debug, Clone)]
pub struct ExerciseLastPerformanceSet {
    pub set_type: SetType,
    pub weight: f32,
    pub reps: u32,
    pub order: u32,
}

/// Recorde pessoal: a série de maior carga já registrada para o exercício.
#[derive(Debug, Clone)]
pub struct ExercisePersonalRecord {
    pub exercise_id: Uuid,
    pub exercise_name: String,
    pub max_weight: f32,
    pub reps: u32,
    pub achieved_at: DateTime<Utc>,
}
