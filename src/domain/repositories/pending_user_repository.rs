use crate::domain::entities::PendingUser;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PendingUserRepository: Send + Sync + 'static {
    async fn create_pending_user(&self, pending_user: PendingUser) -> Result<(), DomainError>;
    async fn get_valid_pending_user_by_email(
        &self,
        email: &str,
    ) -> Result<PendingUser, DomainError>;
    async fn delete_pending_user(&self, id: Uuid) -> Result<(), DomainError>;
    async fn clear_expired_pending_user(&self) -> Result<(), DomainError>;
}
