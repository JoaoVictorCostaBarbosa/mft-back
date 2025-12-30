use crate::{
    application::{
        dtos::user::update_user_request::UpdateUserRequest,
        interfaces::pending_change_repository::PendingChangesRepository,
    },
    domain::{
        commands::user_commands::UserUpdateFilds,
        entities::user::User,
        enums::role::Role,
        errors::{domain_error::DomainError, permission_error::PermissionError},
        repositories::user_repository::UserRepository,
    },
};
use std::sync::Arc;

pub struct UpdateUser {
    pub user_repo: Arc<dyn UserRepository>,
    pub pending_change_repo: Arc<dyn PendingChangesRepository>,
}

impl UpdateUser {
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
        user_data: UpdateUserRequest,
        current_user: User,
    ) -> Result<User, DomainError> {
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

        if pending_change.code != user_data.code {
            return Err(DomainError::Permisson(PermissionError::Forbidden));
        }

        let updated_user: User = self
            .user_repo
            .update_user(
                UserUpdateFilds {
                    name: user_data.name,
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
