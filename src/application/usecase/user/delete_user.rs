use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::enums::Role;
use crate::domain::errors::DomainError;
use crate::domain::errors::PermissionError;
use crate::domain::repositories::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteUser {
    user_repo: Arc<dyn UserRepository>,
}

impl DeleteUser {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, user_id: Uuid, current_user: User) -> Result<(), AppError> {
        if current_user.role != Role::Admin {
            return Err(AppError::Domain(DomainError::Permission(
                PermissionError::Forbidden,
            )));
        }

        self.user_repo.delete_user(user_id).await?;

        Ok(())
    }
}
