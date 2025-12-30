use crate::{
    domain::{
        entities::user::User,
        enums::role::Role,
        errors::{domain_error::DomainError, permission_error::PermissionError},
        repositories::user_repository::UserRepository,
    },
};
use std::sync::Arc;

pub struct FindUsers {
    pub user_repo: Arc<dyn UserRepository>,
}

impl FindUsers {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, current_user: User) -> Result<Vec<User>, DomainError> {
        if current_user.role != Role::Admin {
            return Err(DomainError::Permisson(PermissionError::Forbidden));
        }

        let users = self.user_repo.get_all_users().await?;

        Ok(users)
    }
}
