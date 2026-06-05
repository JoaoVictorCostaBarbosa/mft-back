use crate::domain::{
    enums::{set_type::SetType, workout_session_status::WorkoutSessionStatus},
    errors::workout_log_error::WorkoutLogError,
};
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

pub struct WorkoutSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workout_plan_id: Uuid,
    pub workout_template_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatus,
}

pub struct CurrentWorkoutSession {
    pub id: Uuid,
    pub workout_template_id: Uuid,
    pub workout_template_name: String,
    pub started_at: DateTime<Utc>,
    pub status: WorkoutSessionStatus,
}

pub struct FinishedWorkoutSession {
    pub id: Uuid,
    pub status: WorkoutSessionStatus,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

pub struct WorkoutSessionExercise {
    pub id: Uuid,
    pub workout_session_id: Uuid,
    pub exercise_id: Uuid,
}

pub struct WorkoutSessionSet {
    pub id: Uuid,
    pub session_exercise_id: Uuid,
    pub set_type: SetType,
    pub weight: f32,
    pub reps: u32,
    pub created_at: DateTime<Utc>,
}

pub struct WorkoutSessionHistoryItem {
    pub id: Uuid,
    pub workout_plan_id: Uuid,
    pub workout_template_id: Uuid,
    pub workout_template_name: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: WorkoutSessionStatus,
}

pub struct WorkoutSessionWeeklySummaryDay {
    pub date: NaiveDate,
    pub day_of_week: String,
    pub trained: bool,
    pub session_id: Option<Uuid>,
}

impl WorkoutSession {
    pub fn start(user_id: Uuid, workout_plan_id: Uuid, workout_template_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            workout_plan_id,
            workout_template_id,
            started_at: Utc::now(),
            finished_at: None,
            status: WorkoutSessionStatus::InProgress,
        }
    }

    pub fn finish(
        &self,
        finished_at: Option<DateTime<Utc>>,
    ) -> Result<FinishedWorkoutSession, WorkoutLogError> {
        if self.status == WorkoutSessionStatus::Finished {
            return Err(WorkoutLogError::AlreadyFinished);
        }

        let finished_at = finished_at.unwrap_or_else(Utc::now);

        if finished_at < self.started_at {
            return Err(WorkoutLogError::InvalidFinishedAt);
        }

        Ok(FinishedWorkoutSession {
            id: self.id,
            status: WorkoutSessionStatus::Finished,
            started_at: self.started_at,
            finished_at,
        })
    }
}
