use crate::{
    application::{
        dtos::user::{
            email_change_request::EmailChangeRequest, pending_change::PendingChange,
        },
        interfaces::pending_change_repository::PendingChangesRepository,
    },
    domain::{
        commands::user_commands::UserUpdateFilds, entities::user::User, enums::role::Role, errors::{
            domain_error::DomainError, permission_error::PermissionError, user_error::UserError,
        }, repositories::user_repository::UserRepository, value_objects::email_vo::Email
    },
};
use std::sync::Arc;

pub struct ChangeEmail {
    pub user_repo: Arc<dyn UserRepository>,
    pub pending_change_repo: Arc<dyn PendingChangesRepository>,
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
    ) -> Result<User, DomainError> {
        let email = Email::new(user_data.email).map_err(UserError::EmailInvalid)?;

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

        let pending_change: PendingChange = self
            .pending_change_repo
            .get_valid_pending_change_by_user_id(target_id)
            .await?;

        if user_data.code != pending_change.code {
            return Err(DomainError::Permisson(PermissionError::Forbidden));
        }

        let updated_user = self
            .user_repo
            .update_user(
                UserUpdateFilds {
                    email: Some(email.value().to_string()),
                    ..Default::default()
                },
                target_id,
            )
            .await?;

        self.pending_change_repo
            .delete_pending_change(pending_change.id)
            .await?;

        Ok(updated_user)
    }
}
