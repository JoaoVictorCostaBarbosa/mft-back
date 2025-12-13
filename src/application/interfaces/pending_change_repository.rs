use crate::{
    application::dtos::user::pending_change::PendingChange,
    domain::errors::repository_error::RepositoryError,
};
use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PendingChangesRepository: Send + Sync + 'static {
    async fn create_pending_change(
        &self,
        pending_change: PendingChange,
    ) -> Result<(), RepositoryError>;
    async fn get_valid_pending_change_by_user_id(&self, id: Uuid) -> Result<PendingChange, RepositoryError>;
    async fn delete_pending_change(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn clear_expired_pending_change(&self) -> Result<(), RepositoryError>;
}
