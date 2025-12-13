use crate::domain::{
    entities::user::User,
    enums::role::Role,
    errors::{domain_error::DomainError, permission_error::PermissionError},
    repositories::user_repository::UserRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct SoftDeleteUser {
    pub user_repo: Arc<dyn UserRepository>,
}

impl SoftDeleteUser {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, user_id: Uuid, current_user: User) -> Result<(), DomainError> {
        if current_user.id != user_id && current_user.role != Role::Admin {
            return Err(DomainError::Permisson(PermissionError::Forbidden));
        }

        self.user_repo.soft_delete_user(user_id).await?;

        Ok(())
    }
}
