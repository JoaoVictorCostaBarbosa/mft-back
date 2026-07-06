use crate::application::dtos::auth::VerifyRequest;
use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::errors::PermissionError;
use crate::domain::errors::RepositoryError;
use crate::domain::errors::UserError;
use crate::domain::repositories::PendingUserRepository;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Email;
use std::sync::Arc;

pub struct VerifyUser {
    user_repo: Arc<dyn UserRepository>,
    pending_user_repo: Arc<dyn PendingUserRepository>,
}

impl VerifyUser {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_user_repo: Arc<dyn PendingUserRepository>,
    ) -> Self {
        Self {
            user_repo,
            pending_user_repo,
        }
    }

    pub async fn execute(&self, request: VerifyRequest) -> Result<User, AppError> {
        let email = Email::new(request.email).map_err(UserError::from)?;
        if request.code.to_string().len() < 6 {
            return Err(AppError::Domain(DomainError::Permission(
                PermissionError::Unauthorized,
            )));
        }

        let pending_user = match self
            .pending_user_repo
            .get_valid_pending_user_by_email(email.value())
            .await
        {
            Ok(u) => u,
            Err(DomainError::Repository(RepositoryError::NotFound(_))) => {
                return Err(AppError::Domain(DomainError::Repository(
                    RepositoryError::NotFound("pending user not found".into()),
                )));
            }
            Err(e) => return Err(AppError::Domain(e)),
        };

        if request.code != pending_user.code {
            return Err(AppError::Domain(DomainError::Permission(
                PermissionError::Unauthorized,
            )));
        }

        let user = User::new(pending_user.name, pending_user.email, pending_user.password)?;

        self.user_repo
            .create_user_from_pending(&user, pending_user.id)
            .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::PendingUser;
    use crate::test_support::fakes::InMemoryPendingUserRepository;
    use crate::test_support::fakes::InMemoryUserRepository;
    use chrono::Utc;

    fn pending_user() -> PendingUser {
        PendingUser::new(
            "New User".to_string(),
            "new@test.com".to_string(),
            "hashed:Password1".to_string(),
            123456,
            Utc::now(),
        )
    }

    fn request(code: u32) -> VerifyRequest {
        VerifyRequest {
            email: "new@test.com".to_string(),
            code,
        }
    }

    #[tokio::test]
    async fn correct_code_creates_user_and_removes_pending() {
        let user_repo = Arc::new(InMemoryUserRepository::default());
        let pending_repo = Arc::new(InMemoryPendingUserRepository::with_pending_users(vec![
            pending_user(),
        ]));
        let use_case = VerifyUser::new(user_repo.clone(), pending_repo.clone());

        let user = use_case.execute(request(123456)).await.unwrap();

        assert_eq!(user.email.value(), "new@test.com");
        assert_eq!(user.password, "hashed:Password1");
        assert_eq!(user_repo.users.lock().unwrap().len(), 1);
        assert_eq!(user_repo.consumed_pending_users.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn wrong_code_is_unauthorized_and_creates_nothing() {
        let user_repo = Arc::new(InMemoryUserRepository::default());
        let pending_repo = Arc::new(InMemoryPendingUserRepository::with_pending_users(vec![
            pending_user(),
        ]));
        let use_case = VerifyUser::new(user_repo.clone(), pending_repo.clone());

        let err = use_case.execute(request(654321)).await.unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Unauthorized))
        ));
        assert!(user_repo.users.lock().unwrap().is_empty());
        assert_eq!(pending_repo.pending_users.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn code_with_less_than_six_digits_is_unauthorized() {
        let use_case = VerifyUser::new(
            Arc::new(InMemoryUserRepository::default()),
            Arc::new(InMemoryPendingUserRepository::with_pending_users(vec![
                pending_user(),
            ])),
        );

        let err = use_case.execute(request(12345)).await.unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Unauthorized))
        ));
    }

    #[tokio::test]
    async fn missing_pending_user_is_not_found() {
        let use_case = VerifyUser::new(
            Arc::new(InMemoryUserRepository::default()),
            Arc::new(InMemoryPendingUserRepository::default()),
        );

        let err = use_case.execute(request(123456)).await.unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Repository(RepositoryError::NotFound(_)))
        ));
    }
}
