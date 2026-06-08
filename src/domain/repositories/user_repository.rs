use crate::domain::{
    commands::user_commands::UserUpdateFilds, entities::user::User,
    errors::domain_error::DomainError,
};
use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, user: &User) -> Result<(), DomainError>;
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, DomainError>;
    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError>;
    async fn get_user_by_google_sub(&self, google_sub: &str) -> Result<User, DomainError>;
    async fn link_google_sub(
        &self,
        user_id: Uuid,
        google_sub: &str,
        url_img: Option<String>,
    ) -> Result<User, DomainError>;
    async fn get_all_users(&self) -> Result<Vec<User>, DomainError>;
    async fn update_user(&self, user: UserUpdateFilds, user_id: Uuid) -> Result<User, DomainError>;
    async fn soft_delete_user(&self, user_id: Uuid) -> Result<(), DomainError>;
    async fn restore_user(&self, user_id: Uuid) -> Result<(), DomainError>;
    async fn delete_user(&self, user_id: Uuid) -> Result<(), DomainError>;
}
