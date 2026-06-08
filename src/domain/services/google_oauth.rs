use crate::domain::errors::domain_error::DomainError;
use axum::async_trait;

#[derive(Debug, Clone)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

#[async_trait]
pub trait GoogleOAuthProvider: Send + Sync {
    async fn verify_id_token(&self, id_token: &str) -> Result<GoogleUserInfo, DomainError>;
}
