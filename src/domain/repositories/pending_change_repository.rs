use crate::domain::entities::PendingChange;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PendingChangesRepository: Send + Sync + 'static {
    async fn create_pending_change(&self, pending_change: PendingChange)
    -> Result<(), DomainError>;
    async fn get_valid_pending_change_by_user_id(
        &self,
        id: Uuid,
    ) -> Result<PendingChange, DomainError>;
    async fn delete_pending_change(&self, id: Uuid) -> Result<(), DomainError>;
    async fn clear_expired_pending_change(&self) -> Result<(), DomainError>;
}
