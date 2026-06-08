use crate::{
    application::dtos::auth::google_login_request::GoogleLoginRequest,
    domain::{
        entities::user::User,
        errors::{
            domain_error::DomainError, repository_error::RepositoryError, user_error::UserError,
        },
        repositories::user_repository::UserRepository,
        services::{cripto::CriptoService, google_oauth::GoogleOAuthProvider},
        value_objects::email_vo::Email,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct LoginWithGoogle {
    user_repo: Arc<dyn UserRepository>,
    cripto_service: Arc<dyn CriptoService>,
    google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
}

impl LoginWithGoogle {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        cripto_service: Arc<dyn CriptoService>,
        google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
    ) -> Self {
        Self {
            user_repo,
            cripto_service,
            google_oauth_provider,
        }
    }

    pub async fn execute(&self, request: GoogleLoginRequest) -> Result<User, DomainError> {
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
            Err(e) => return Err(e),
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

                self.user_repo
                    .link_google_sub(user.id, &google_user.sub, google_user.picture)
                    .await
            }
            Err(DomainError::Repository(RepositoryError::NotFound(_))) => {
                let password = self.cripto_service.hash(&Uuid::new_v4().to_string())?;
                let mut user = User::new(google_user.name, google_user.email, password)?;
                user.google_sub = Some(google_user.sub);
                user.url_img = google_user.picture;

                self.user_repo.create_user(&user).await?;

                Ok(user)
            }
            Err(e) => Err(e),
        }
    }
}
