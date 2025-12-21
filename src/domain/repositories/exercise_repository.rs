use crate::domain::{
    commands::exercise_commands::{ExerciseFilterFields, ExerciseUpdateFields},
    entities::exercise::Exercise,
    errors::domain_error::DomainError,
};
use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ExerciseRepository: Send + Sync + 'static {
    async fn create_exercise(&self, exercise: &Exercise) -> Result<(), DomainError>;
    async fn get_exercises(
        &self,
        fields: ExerciseFilterFields,
    ) -> Result<Vec<Exercise>, DomainError>;
    async fn update_exercise(&self, fields: ExerciseUpdateFields) -> Result<(), DomainError>;
    async fn soft_delete_exercise(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_exercise(&self, id: Uuid) -> Result<(), DomainError>;
}
