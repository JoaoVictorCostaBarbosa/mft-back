use crate::{
    application::{
        dtos::user::password_change_request::PasswordChangeRequest,
        interfaces::pending_change_repository::PendingChangesRepository,
    },
    domain::{
        entities::user::User,
        enums::role::Role,
        errors::{
            domain_error::DomainError, permission_error::PermissionError, user_error::UserError,
        },
        repositories::user_repository::UserRepository,
        services::cripto::CriptoService,
        value_objects::{password_vo::Password, user_update::UserUpdateFilds},
    },
};
use std::sync::Arc;

pub struct ChangePassword {
    pub user_repo: Arc<dyn UserRepository>,
    pub pending_change_repo: Arc<dyn PendingChangesRepository>,
    pub cripto_service: Arc<dyn CriptoService>,
}

impl ChangePassword {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_change_repo: Arc<dyn PendingChangesRepository>,
        cripto_service: Arc<dyn CriptoService>,
    ) -> Self {
        Self {
            user_repo,
            pending_change_repo,
            cripto_service,
        }
    }

    pub async fn execute(
        &self,
        user_data: PasswordChangeRequest,
        current_user: User,
    ) -> Result<(), DomainError> {
        let password = Password::new(user_data.password).map_err(UserError::PasswordInvalid)?;

        let target_id = match user_data.id {
            None => current_user.id,
            Some(id) => {
                if current_user.role != Role::Admin {
                    return Err(DomainError::Permisson(PermissionError::Forbidden));
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
            return Err(DomainError::Permisson(PermissionError::Forbidden));
        }

        let password = self.cripto_service.hash(password.value())?;

        self.user_repo
            .update_user(
                UserUpdateFilds {
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
