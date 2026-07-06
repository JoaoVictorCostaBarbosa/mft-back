use crate::domain::commands::ExerciseFilterFields;
use crate::domain::commands::ExerciseUpdateFields;
use crate::domain::entities::Exercise;
use crate::domain::entities::Paginated;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ExerciseRepository: Send + Sync + 'static {
    async fn create_exercise(&self, exercise: &Exercise) -> Result<(), DomainError>;
    async fn get_exercises(
        &self,
        fields: ExerciseFilterFields,
    ) -> Result<Paginated<Exercise>, DomainError>;
    async fn read_by_id(&self, exercise_id: Uuid) -> Result<Exercise, DomainError>;
    async fn update_exercise(
        &self,
        fields: ExerciseUpdateFields,
        user_id: Option<Uuid>,
    ) -> Result<(), DomainError>;
    async fn soft_delete_exercise(&self, id: Uuid, user_id: Uuid) -> Result<(), DomainError>;
    async fn delete_exercise(&self, id: Uuid) -> Result<(), DomainError>;
}
