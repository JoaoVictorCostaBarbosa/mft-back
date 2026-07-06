use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::errors::PermissionError;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetAuthenticatedUser {
    user_repo: Arc<dyn UserRepository>,
}

impl GetAuthenticatedUser {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<User, AppError> {
        let user = match self.user_repo.get_user_by_id(user_id).await {
            Ok(user) => user,
            Err(DomainError::Repository(RepositoryError::NotFound(_))) => {
                return Err(AppError::Domain(DomainError::Permission(
                    PermissionError::Forbidden,
                )));
            }
            Err(e) => return Err(AppError::Domain(e)),
        };

        if user.deleted_at.is_some() {
            return Err(AppError::Domain(DomainError::Permission(
                PermissionError::Forbidden,
            )));
        }

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::fakes::InMemoryUserRepository;
    use crate::test_support::fixtures;
    use chrono::Utc;

    #[tokio::test]
    async fn active_user_is_returned() {
        let user = fixtures::user();
        let user_id = user.id;
        let use_case =
            GetAuthenticatedUser::new(Arc::new(InMemoryUserRepository::with_users(vec![user])));

        let authenticated = use_case.execute(user_id).await.unwrap();

        assert_eq!(authenticated.id, user_id);
    }

    #[tokio::test]
    async fn soft_deleted_user_is_forbidden() {
        let mut user = fixtures::user();
        user.deleted_at = Some(Utc::now());
        let user_id = user.id;
        let use_case =
            GetAuthenticatedUser::new(Arc::new(InMemoryUserRepository::with_users(vec![user])));

        let err = use_case.execute(user_id).await.unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
    }

    #[tokio::test]
    async fn unknown_user_is_forbidden() {
        let use_case = GetAuthenticatedUser::new(Arc::new(InMemoryUserRepository::default()));

        let err = use_case.execute(Uuid::new_v4()).await.unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
    }
}
