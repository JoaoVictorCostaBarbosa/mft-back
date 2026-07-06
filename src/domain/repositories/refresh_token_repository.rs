use crate::domain::entities::RefreshToken;
use crate::domain::errors::RepositoryError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(&self, token: RefreshToken) -> Result<(), RepositoryError>;
    async fn find_valid_by_hash(&self, hash: &str) -> Result<RefreshToken, RepositoryError>;
    async fn revoke(&self, token_id: Uuid) -> Result<(), RepositoryError>;
    /// Revoga o token antigo e persiste o novo na mesma transação.
    async fn rotate(
        &self,
        revoked_id: Uuid,
        new_token: RefreshToken,
    ) -> Result<(), RepositoryError>;
}
