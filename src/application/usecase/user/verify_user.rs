use crate::{
    application::{
        dtos::user::{
            auth_reponse::AuthResponse, user_response::UserResponse, verify_request::VerifyRequest,
        },
        interfaces::pending_user_repository::PendingUserRepository,
    },
    domain::{
        entities::user::User,
        errors::{
            domain_error::DomainError, permission_error::PermissionError,
            repository_error::RepositoryError, user_error::UserError,
        },
        repositories::user_repository::UserRepository,
        services::jwt::JwtProvider,
        value_objects::email_vo::Email,
    },
};
use std::sync::Arc;

pub struct VerifyUser {
    user_repo: Arc<dyn UserRepository>,
    pending_user_repo: Arc<dyn PendingUserRepository>,
    jwt_service: Arc<dyn JwtProvider>,
}

impl VerifyUser {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_user_repo: Arc<dyn PendingUserRepository>,
        jwt_service: Arc<dyn JwtProvider>,
    ) -> Self {
        Self {
            user_repo,
            pending_user_repo,
            jwt_service,
        }
    }

    pub async fn execute(&self, request: VerifyRequest) -> Result<AuthResponse, DomainError> {
        let email = Email::new(request.email).map_err(UserError::from)?;
        if request.code.to_string().len() < 6 {
            return Err(DomainError::Permisson(PermissionError::Unauthorized));
        }

        let pending_user = match self
            .pending_user_repo
            .get_valid_pending_user_by_email(email.value())
            .await
        {
            Ok(u) => u,
            Err(RepositoryError::NotFound(_)) => {
                return Err(DomainError::Repository(RepositoryError::NotFound(
                    "pending user not found".into(),
                )));
            }
            Err(e) => return Err(DomainError::Repository(e)),
        };

        if request.code != pending_user.code {
            return Err(DomainError::Permisson(PermissionError::Unauthorized));
        }

        let user = User::new(pending_user.name, pending_user.email, pending_user.password)?;

        (self.user_repo.create_user(&user)).await?;

        let access = self
            .jwt_service
            .generate_access(user.id.to_string(), user.role)?;
        let refresh = self.jwt_service.generate_refresh(user.id.to_string())?;

        (self.pending_user_repo.delete_pending_user(pending_user.id)).await?;

        let response = AuthResponse::new(UserResponse::to_response(user), access, refresh);

        Ok(response)
    }
}
