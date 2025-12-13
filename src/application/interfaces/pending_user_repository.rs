use crate::{
    application::dtos::user::pending_user::PendingUser,
    domain::errors::repository_error::RepositoryError,
};
use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PendingUserRepository: Send + Sync + 'static {
    async fn create_pending_user(&self, pending_user: PendingUser) -> Result<(), RepositoryError>;
    async fn get_valid_pending_user_by_email(&self, email: &str) -> Result<PendingUser, RepositoryError>;
    async fn delete_pending_user(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn clear_expired_pending_user(&self) -> Result<(), RepositoryError>;
}
