use crate::application::errors::AppError;
use crate::application::ports::Clock;
use crate::application::ports::RefreshTokenHasher;
use crate::application::ports::TokenGenerator;
use crate::domain::entities::RefreshToken;
use crate::domain::repositories::RefreshTokenRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct IssueRefreshToken {
    refresh_repo: Arc<dyn RefreshTokenRepository>,
    hash_service: Arc<dyn RefreshTokenHasher>,
    token_generator: Arc<dyn TokenGenerator>,
    clock: Arc<dyn Clock>,
    pub refresh_exp_in_days: i64,
}

impl IssueRefreshToken {
    pub fn new(
        refresh_repo: Arc<dyn RefreshTokenRepository>,
        hash_service: Arc<dyn RefreshTokenHasher>,
        token_generator: Arc<dyn TokenGenerator>,
        clock: Arc<dyn Clock>,
        refresh_exp_in_days: i64,
    ) -> Self {
        Self {
            refresh_repo,
            hash_service,
            token_generator,
            clock,
            refresh_exp_in_days,
        }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<String, AppError> {
        let refresh_raw = self.token_generator.generate();
        let refresh_hash = self.hash_service.hash(&refresh_raw)?;

        let refresh = RefreshToken::new(
            user_id,
            refresh_hash,
            self.refresh_exp_in_days,
            self.clock.now(),
        );

        self.refresh_repo.create(refresh).await?;

        Ok(refresh_raw)
    }
}
