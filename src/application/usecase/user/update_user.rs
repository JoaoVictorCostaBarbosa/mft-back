use crate::application::dtos::user::UpdateUserRequest;
use crate::application::errors::AppError;
use crate::domain::commands::UserUpdateFields;
use crate::domain::entities::User;
use crate::domain::enums::Role;
use crate::domain::errors::DomainError;
use crate::domain::errors::PermissionError;
use crate::domain::repositories::UserRepository;
use std::sync::Arc;

pub struct UpdateUser {
    user_repo: Arc<dyn UserRepository>,
}

impl UpdateUser {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(
        &self,
        user_data: UpdateUserRequest,
        current_user: User,
    ) -> Result<User, AppError> {
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

        let updated_user: User = self
            .user_repo
            .update_user(
                UserUpdateFields {
                    name: user_data.name,
                    ..Default::default()
                },
                target_id,
            )
            .await?;

        Ok(updated_user)
    }
}
