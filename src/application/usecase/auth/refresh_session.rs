use crate::application::dtos::auth::RefreshResponse;
use crate::application::errors::AppError;
use crate::application::ports::Clock;
use crate::application::ports::JwtProvider;
use crate::application::ports::RefreshTokenHasher;
use crate::application::ports::TokenGenerator;
use crate::domain::entities::RefreshToken;
use crate::domain::errors::PermissionError;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::RefreshTokenRepository;
use crate::domain::repositories::UserRepository;
use std::sync::Arc;

pub struct RefreshSession {
    refresh_token_repo: Arc<dyn RefreshTokenRepository>,
    user_repo: Arc<dyn UserRepository>,
    jwt_service: Arc<dyn JwtProvider>,
    hash_service: Arc<dyn RefreshTokenHasher>,
    token_generator: Arc<dyn TokenGenerator>,
    clock: Arc<dyn Clock>,
    pub refresh_exp_in_days: i64,
}

impl RefreshSession {
    pub fn new(
        refresh_token_repo: Arc<dyn RefreshTokenRepository>,
        user_repo: Arc<dyn UserRepository>,
        jwt_service: Arc<dyn JwtProvider>,
        hash_service: Arc<dyn RefreshTokenHasher>,
        token_generator: Arc<dyn TokenGenerator>,
        clock: Arc<dyn Clock>,
        refresh_exp_in_days: i64,
    ) -> Self {
        Self {
            refresh_token_repo,
            user_repo,
            jwt_service,
            hash_service,
            token_generator,
            clock,
            refresh_exp_in_days,
        }
    }

    pub async fn execute(&self, token: String) -> Result<RefreshResponse, AppError> {
        let hashed_token = self.hash_service.hash(&token)?;

        let refresh_token = match self
            .refresh_token_repo
            .find_valid_by_hash(&hashed_token)
            .await
        {
            Ok(t) => t,
            Err(RepositoryError::NotFound(_)) => return Err(PermissionError::Unauthorized.into()),
            Err(e) => return Err(e.into()),
        };

        let user = self.user_repo.get_user_by_id(refresh_token.user_id).await?;

        let access = self
            .jwt_service
            .generate_access(user.id.to_string(), user.role)?;

        let refresh_raw = self.token_generator.generate();
        let refresh_hash = self.hash_service.hash(&refresh_raw)?;

        let refresh = RefreshToken::new(
            user.id,
            refresh_hash,
            self.refresh_exp_in_days,
            self.clock.now(),
        );

        // Revogação do token antigo e criação do novo na mesma transação.
        self.refresh_token_repo
            .rotate(refresh_token.id, refresh)
            .await?;

        let response = RefreshResponse::new(access, refresh_raw);

        Ok(response)
    }
}
