use crate::application::dtos::user::EmailChangeRequest;
use crate::application::errors::AppError;
use crate::domain::entities::PendingChange;
use crate::domain::entities::User;
use crate::domain::enums::Role;
use crate::domain::errors::DomainError;
use crate::domain::errors::PermissionError;
use crate::domain::errors::UserError;
use crate::domain::repositories::PendingChangesRepository;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Email;
use std::sync::Arc;

pub struct ChangeEmail {
    user_repo: Arc<dyn UserRepository>,
    pending_change_repo: Arc<dyn PendingChangesRepository>,
}

impl ChangeEmail {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_change_repo: Arc<dyn PendingChangesRepository>,
    ) -> Self {
        Self {
            user_repo,
            pending_change_repo,
        }
    }

    pub async fn execute(
        &self,
        user_data: EmailChangeRequest,
        current_user: User,
    ) -> Result<User, AppError> {
        let email = Email::new(user_data.email).map_err(UserError::EmailInvalid)?;

        let target_id = match user_data.id {
            None => current_user.id,
            Some(id) => {
                if current_user.role != Role::Admin {
                    return Err(AppError::Domain(DomainError::Permission(
                        PermissionError::Forbidden,
                    )));
                }
                id
            }
        };

        self.user_repo.get_user_by_id(target_id).await?;

        let pending_change: PendingChange = self
            .pending_change_repo
            .get_valid_pending_change_by_user_id(target_id)
            .await?;

        if user_data.code != pending_change.code {
            return Err(AppError::Domain(DomainError::Permission(
                PermissionError::Forbidden,
            )));
        }

        let updated_user = self
            .user_repo
            .apply_email_change(target_id, email.value(), pending_change.id)
            .await?;

        Ok(updated_user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::PendingChange;
    use crate::test_support::fakes::InMemoryPendingChangeRepository;
    use crate::test_support::fakes::InMemoryUserRepository;
    use crate::test_support::fixtures;
    use chrono::Utc;
    use std::sync::Arc;

    #[tokio::test]
    async fn valid_code_changes_email_and_consumes_change() {
        let user = fixtures::user();
        let pending = PendingChange::new(user.id, 123456, Utc::now());
        let user_repo = Arc::new(InMemoryUserRepository::with_users(vec![
            fixtures::clone_user(&user),
        ]));
        let change_repo = Arc::new(InMemoryPendingChangeRepository::with_pending_changes(vec![
            pending,
        ]));
        let use_case = ChangeEmail::new(user_repo.clone(), change_repo);

        let updated = use_case
            .execute(
                EmailChangeRequest {
                    id: None,
                    email: "novo@test.com".to_string(),
                    code: 123456,
                },
                user,
            )
            .await
            .unwrap();

        assert_eq!(updated.email.value(), "novo@test.com");
        assert_eq!(user_repo.consumed_pending_changes.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn wrong_code_is_forbidden() {
        let user = fixtures::user();
        let pending = PendingChange::new(user.id, 123456, Utc::now());
        let user_repo = Arc::new(InMemoryUserRepository::with_users(vec![
            fixtures::clone_user(&user),
        ]));
        let change_repo = Arc::new(InMemoryPendingChangeRepository::with_pending_changes(vec![
            pending,
        ]));
        let use_case = ChangeEmail::new(user_repo.clone(), change_repo);

        let err = use_case
            .execute(
                EmailChangeRequest {
                    id: None,
                    email: "novo@test.com".to_string(),
                    code: 999999,
                },
                user,
            )
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
        assert!(
            user_repo
                .consumed_pending_changes
                .lock()
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn non_admin_cannot_change_another_user_email() {
        let user = fixtures::user();
        let use_case = ChangeEmail::new(
            Arc::new(InMemoryUserRepository::default()),
            Arc::new(InMemoryPendingChangeRepository::default()),
        );

        let err = use_case
            .execute(
                EmailChangeRequest {
                    id: Some(uuid::Uuid::new_v4()),
                    email: "novo@test.com".to_string(),
                    code: 123456,
                },
                user,
            )
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
    }
}
