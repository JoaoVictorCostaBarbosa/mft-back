use crate::application::read_models::ExerciseLastPerformance;
use crate::application::read_models::ExercisePersonalRecord;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

/// Port de consulta (lado de leitura) dos exercícios.
#[async_trait]
pub trait ExerciseQueries: Send + Sync {
    async fn find_last_performances(
        &self,
        user_id: Uuid,
        exercise_ids: Vec<Uuid>,
    ) -> Result<Vec<ExerciseLastPerformance>, DomainError>;

    async fn find_personal_records(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ExercisePersonalRecord>, DomainError>;
}
