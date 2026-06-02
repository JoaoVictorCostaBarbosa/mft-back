use crate::domain::{
    commands::workout_template_command::WorkoutTemplateFilterFields,
    entities::workout_template::{WorkoutTemplate, WorkoutTemplateSummary},
    errors::domain_error::DomainError,
};
use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait WorkoutTemplateRepository: Send + Sync {
    async fn save(&self, workout: &WorkoutTemplate) -> Result<(), DomainError>;
    async fn read(
        &self,
        fields: WorkoutTemplateFilterFields,
    ) -> Result<Vec<WorkoutTemplateSummary>, DomainError>;
    async fn find_by_id(&self, workout_id: Uuid) -> Result<WorkoutTemplate, DomainError>;
    async fn update(&self, workout: &WorkoutTemplate) -> Result<(), DomainError>;
    async fn soft_delete(&self, workout_id: Uuid) -> Result<(), DomainError>;
    async fn delete(&self, workout_id: Uuid) -> Result<(), DomainError>;
    async fn add_exercise(&self, workout_id: Uuid, exercise_id: Uuid) -> Result<(), DomainError>;
    async fn remove_exercise(&self, workout_id: Uuid, exercise_id: Uuid)
    -> Result<(), DomainError>;
}
