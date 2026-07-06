use crate::application::read_models::CurrentWorkoutSession;
use crate::application::read_models::WorkoutSessionHistoryItem;
use crate::application::read_models::WorkoutSessionWeeklySummary;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

/// Port de consulta (lado de leitura) das sessões de treino.
#[async_trait]
pub trait WorkoutSessionQueries: Send + Sync {
    async fn find_current(&self, user_id: Uuid) -> Result<CurrentWorkoutSession, DomainError>;
    async fn history(&self, user_id: Uuid) -> Result<Vec<WorkoutSessionHistoryItem>, DomainError>;
    async fn weekly_summary(
        &self,
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<WorkoutSessionWeeklySummary, DomainError>;
}
