use crate::application::errors::AppError;
use crate::application::ports::RefreshTokenHasher;
use crate::domain::errors::PermissionError;
use crate::domain::repositories::RefreshTokenRepository;
use std::sync::Arc;

pub struct Logout {
    refresh_repo: Arc<dyn RefreshTokenRepository>,
    hash_service: Arc<dyn RefreshTokenHasher>,
}

impl Logout {
    pub fn new(
        refresh_repo: Arc<dyn RefreshTokenRepository>,
        hash_service: Arc<dyn RefreshTokenHasher>,
    ) -> Self {
        Self {
            refresh_repo,
            hash_service,
        }
    }

    pub async fn execute(&self, token: String) -> Result<(), AppError> {
        let hashed_token = self.hash_service.hash(&token)?;

        let refresh_token = match self.refresh_repo.find_valid_by_hash(&hashed_token).await {
            Ok(t) => t,
            Err(_) => return Err(PermissionError::Unauthorized.into()),
        };

        self.refresh_repo.revoke(refresh_token.id).await?;

        Ok(())
    }
}
