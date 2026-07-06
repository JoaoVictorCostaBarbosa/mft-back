use crate::application::dtos::user::PasswordChangeRequest;
use crate::application::errors::AppError;
use crate::application::ports::CryptoService;
use crate::domain::commands::UserUpdateFields;
use crate::domain::entities::User;
use crate::domain::enums::Role;
use crate::domain::errors::DomainError;
use crate::domain::errors::PermissionError;
use crate::domain::errors::UserError;
use crate::domain::repositories::PendingChangesRepository;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Password;
use std::sync::Arc;

pub struct ChangePassword {
    user_repo: Arc<dyn UserRepository>,
    pending_change_repo: Arc<dyn PendingChangesRepository>,
    crypto_service: Arc<dyn CryptoService>,
}

impl ChangePassword {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_change_repo: Arc<dyn PendingChangesRepository>,
        crypto_service: Arc<dyn CryptoService>,
    ) -> Self {
        Self {
            user_repo,
            pending_change_repo,
            crypto_service,
        }
    }

    pub async fn execute(
        &self,
        user_data: PasswordChangeRequest,
        current_user: User,
    ) -> Result<(), AppError> {
        let password = Password::new(user_data.password).map_err(UserError::PasswordInvalid)?;

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

        let pending_change = self
            .pending_change_repo
            .get_valid_pending_change_by_user_id(target_id)
            .await?;

        if user_data.code != pending_change.code {
            return Err(AppError::Domain(DomainError::Permission(
                PermissionError::Forbidden,
            )));
        }

        let password = self.crypto_service.hash(password.value())?;

        self.user_repo
            .update_user(
                UserUpdateFields {
                    password: Some(password),
                    ..Default::default()
                },
                target_id,
            )
            .await?;

        self.pending_change_repo
            .delete_pending_change(pending_change.id)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::PendingChange;
    use crate::test_support::fakes::FakeCryptoService;
    use crate::test_support::fakes::InMemoryPendingChangeRepository;
    use crate::test_support::fakes::InMemoryUserRepository;
    use crate::test_support::fixtures;
    use chrono::Utc;

    #[tokio::test]
    async fn valid_code_updates_password_hash() {
        let user = fixtures::user();
        let user_id = user.id;
        let pending = PendingChange::new(user.id, 123456, Utc::now());
        let user_repo = Arc::new(InMemoryUserRepository::with_users(vec![
            fixtures::clone_user(&user),
        ]));
        let change_repo = Arc::new(InMemoryPendingChangeRepository::with_pending_changes(vec![
            pending,
        ]));
        let use_case = ChangePassword::new(
            user_repo.clone(),
            change_repo.clone(),
            Arc::new(FakeCryptoService),
        );

        use_case
            .execute(
                PasswordChangeRequest {
                    id: None,
                    password: "NovaSenha1".to_string(),
                    code: 123456,
                },
                user,
            )
            .await
            .unwrap();

        let users = user_repo.users.lock().unwrap();
        let stored = users.iter().find(|u| u.id == user_id).unwrap();
        assert_eq!(stored.password, "hashed:NovaSenha1");
        assert!(change_repo.pending_changes.lock().unwrap().is_empty());
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
        let use_case =
            ChangePassword::new(user_repo.clone(), change_repo, Arc::new(FakeCryptoService));

        let err = use_case
            .execute(
                PasswordChangeRequest {
                    id: None,
                    password: "NovaSenha1".to_string(),
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
    }
}
