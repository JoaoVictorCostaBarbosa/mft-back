use crate::application::dtos::auth::GoogleLoginRequest;
use crate::application::errors::AppError;
use crate::application::ports::CryptoService;
use crate::application::ports::GoogleOAuthProvider;
use crate::application::ports::TokenGenerator;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::errors::RepositoryError;
use crate::domain::errors::UserError;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Email;
use std::sync::Arc;

pub struct LoginWithGoogle {
    user_repo: Arc<dyn UserRepository>,
    crypto_service: Arc<dyn CryptoService>,
    google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
    token_generator: Arc<dyn TokenGenerator>,
}

impl LoginWithGoogle {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        crypto_service: Arc<dyn CryptoService>,
        google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
        token_generator: Arc<dyn TokenGenerator>,
    ) -> Self {
        Self {
            user_repo,
            crypto_service,
            google_oauth_provider,
            token_generator,
        }
    }

    pub async fn execute(&self, request: GoogleLoginRequest) -> Result<User, AppError> {
        let google_user = self
            .google_oauth_provider
            .verify_id_token(&request.id_token)
            .await?;

        let _email = Email::new(google_user.email.clone())
            .map_err(|e| DomainError::User(UserError::EmailInvalid(e)))?;

        match self
            .user_repo
            .get_user_by_google_sub(&google_user.sub)
            .await
        {
            Ok(user) => return Ok(user),
            Err(DomainError::Repository(RepositoryError::NotFound(_))) => {}
            Err(e) => return Err(AppError::Domain(e)),
        }

        match self.user_repo.get_user_by_email(&google_user.email).await {
            Ok(user) => {
                if user.google_sub.as_deref() != Some(google_user.sub.as_str())
                    && user.google_sub.is_some()
                {
                    return Err(RepositoryError::Conflict(
                        "email already linked to another google account".to_string(),
                    )
                    .into());
                }

                Ok(self
                    .user_repo
                    .link_google_sub(user.id, &google_user.sub, google_user.picture)
                    .await?)
            }
            Err(DomainError::Repository(RepositoryError::NotFound(_))) => {
                let password = self.crypto_service.hash(&self.token_generator.generate())?;
                let mut user = User::new(google_user.name, google_user.email, password)?;
                user.google_sub = Some(google_user.sub);
                user.url_img = google_user.picture;

                self.user_repo.create_user(&user).await?;

                Ok(user)
            }
            Err(e) => Err(AppError::Domain(e)),
        }
    }
}
