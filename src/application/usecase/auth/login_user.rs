use crate::application::dtos::auth::LoginRequest;
use crate::application::errors::AppError;
use crate::application::ports::CryptoService;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::errors::PermissionError;
use crate::domain::errors::UserError;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Email;
use std::sync::Arc;

pub struct LoginUser {
    user_repo: Arc<dyn UserRepository>,
    crypto_service: Arc<dyn CryptoService>,
}

impl LoginUser {
    pub fn new(user_repo: Arc<dyn UserRepository>, crypto_service: Arc<dyn CryptoService>) -> Self {
        Self {
            user_repo,
            crypto_service,
        }
    }

    pub async fn execute(&self, user_data: LoginRequest) -> Result<User, AppError> {
        let _email = Email::new(user_data.email.clone())
            .map_err(|e| DomainError::User(UserError::EmailInvalid(e)))?;

        let user: User = self
            .user_repo
            .get_user_by_email(user_data.email.as_str())
            .await
            .map_err(|_| DomainError::Permission(PermissionError::Unauthorized))?;

        if user.deleted_at.is_some() {
            return Err(AppError::Domain(DomainError::Permission(
                PermissionError::Unauthorized,
            )));
        }

        if !(self
            .crypto_service
            .verify(&user_data.password, &user.password))?
        {
            return Err(AppError::Domain(DomainError::Permission(
                PermissionError::Unauthorized,
            )));
        }

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::fakes::FakeCryptoService;
    use crate::test_support::fakes::InMemoryUserRepository;
    use crate::test_support::fixtures;
    use chrono::Utc;

    fn use_case(user_repo: Arc<InMemoryUserRepository>) -> LoginUser {
        LoginUser::new(user_repo, Arc::new(FakeCryptoService))
    }

    fn request(email: &str, password: &str) -> LoginRequest {
        LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        }
    }

    #[tokio::test]
    async fn valid_credentials_return_user() {
        let user = fixtures::user_with_email("user@test.com");
        let expected_id = user.id;
        let use_case = use_case(Arc::new(InMemoryUserRepository::with_users(vec![user])));

        let logged = use_case
            .execute(request("user@test.com", "Password1"))
            .await
            .unwrap();

        assert_eq!(logged.id, expected_id);
    }

    #[tokio::test]
    async fn wrong_password_is_unauthorized() {
        let user = fixtures::user_with_email("user@test.com");
        let use_case = use_case(Arc::new(InMemoryUserRepository::with_users(vec![user])));

        let err = use_case
            .execute(request("user@test.com", "WrongPass1"))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Unauthorized))
        ));
    }

    #[tokio::test]
    async fn unknown_email_is_unauthorized() {
        let use_case = use_case(Arc::new(InMemoryUserRepository::default()));

        let err = use_case
            .execute(request("ghost@test.com", "Password1"))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Unauthorized))
        ));
    }

    #[tokio::test]
    async fn soft_deleted_user_cannot_login() {
        let mut user = fixtures::user_with_email("user@test.com");
        user.deleted_at = Some(Utc::now());
        let use_case = use_case(Arc::new(InMemoryUserRepository::with_users(vec![user])));

        let err = use_case
            .execute(request("user@test.com", "Password1"))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Unauthorized))
        ));
    }
}
