use crate::domain::{
    entities::workout_session::{
        CurrentWorkoutSession, FinishedWorkoutSession, WorkoutSession, WorkoutSessionExercise,
        WorkoutSessionHistoryItem, WorkoutSessionSet, WorkoutSessionWeeklySummaryDay,
    },
    enums::set_type::SetType,
    errors::domain_error::DomainError,
};
use axum::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[async_trait]
pub trait WorkoutSessionRepository: Send + Sync {
    async fn start(&self, session: &WorkoutSession) -> Result<(), DomainError>;
    async fn find_current(&self, user_id: Uuid) -> Result<CurrentWorkoutSession, DomainError>;
    async fn find_by_id(&self, session_id: Uuid) -> Result<WorkoutSession, DomainError>;
    async fn finish(&self, session: &FinishedWorkoutSession) -> Result<(), DomainError>;
    async fn cancel(&self, session_id: Uuid) -> Result<(), DomainError>;
    async fn add_exercise(
        &self,
        session_id: Uuid,
        exercise_id: Uuid,
        client_operation_id: Option<Uuid>,
    ) -> Result<WorkoutSessionExercise, DomainError>;
    async fn add_set(
        &self,
        session_id: Uuid,
        exercise_id: Uuid,
        set_type: SetType,
        weight: f32,
        reps: u32,
        client_operation_id: Option<Uuid>,
        completed_at: Option<DateTime<Utc>>,
    ) -> Result<WorkoutSessionSet, DomainError>;
    async fn update_set(
        &self,
        session_id: Uuid,
        set_id: Uuid,
        set_type: SetType,
        weight: f32,
        reps: u32,
    ) -> Result<WorkoutSessionSet, DomainError>;
    async fn delete_set(&self, session_id: Uuid, set_id: Uuid) -> Result<(), DomainError>;
    async fn reorder_exercises(
        &self,
        session_id: Uuid,
        ordered_session_exercise_ids: Vec<Uuid>,
    ) -> Result<(), DomainError>;
    async fn find_session_exercise_ids(&self, session_id: Uuid) -> Result<Vec<Uuid>, DomainError>;
    async fn remove_exercise(
        &self,
        session_id: Uuid,
        session_exercise_id: Uuid,
    ) -> Result<(), DomainError>;
    async fn history(&self, user_id: Uuid) -> Result<Vec<WorkoutSessionHistoryItem>, DomainError>;
    async fn weekly_summary(
        &self,
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<WorkoutSessionWeeklySummaryDay>, DomainError>;
    async fn has_in_progress(&self, user_id: Uuid) -> Result<bool, DomainError>;
}
