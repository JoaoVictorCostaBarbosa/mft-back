use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::enums::Role;
use crate::domain::errors::DomainError;
use crate::domain::errors::PermissionError;
use crate::domain::repositories::UserRepository;
use std::sync::Arc;

pub struct FindUsers {
    user_repo: Arc<dyn UserRepository>,
}

impl FindUsers {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, current_user: User) -> Result<Vec<User>, AppError> {
        if current_user.role != Role::Admin {
            return Err(AppError::Domain(DomainError::Permission(
                PermissionError::Forbidden,
            )));
        }

        let users = self.user_repo.get_all_users().await?;

        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::fakes::InMemoryUserRepository;
    use crate::test_support::fixtures;
    use std::sync::Arc;

    #[tokio::test]
    async fn non_admin_cannot_list_users() {
        let use_case = FindUsers::new(Arc::new(InMemoryUserRepository::default()));

        let err = use_case.execute(fixtures::user()).await.unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
    }

    #[tokio::test]
    async fn admin_lists_all_users() {
        let repo = Arc::new(InMemoryUserRepository::with_users(vec![
            fixtures::user(),
            fixtures::user_with_email("b@test.com"),
        ]));
        let use_case = FindUsers::new(repo);

        let users = use_case.execute(fixtures::admin()).await.unwrap();

        assert_eq!(users.len(), 2);
    }
}
