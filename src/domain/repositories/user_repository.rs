use crate::domain::commands::UserUpdateFields;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, user: &User) -> Result<(), DomainError>;
    /// Cria o usuário e consome (remove) o cadastro pendente na mesma transação.
    async fn create_user_from_pending(
        &self,
        user: &User,
        pending_user_id: Uuid,
    ) -> Result<(), DomainError>;
    /// Aplica a troca de e-mail e consome o código pendente na mesma transação.
    async fn apply_email_change(
        &self,
        user_id: Uuid,
        email: &str,
        pending_change_id: Uuid,
    ) -> Result<User, DomainError>;
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
    async fn update_user(&self, user: UserUpdateFields, user_id: Uuid)
    -> Result<User, DomainError>;
    async fn soft_delete_user(&self, user_id: Uuid) -> Result<(), DomainError>;
    async fn restore_user(&self, user_id: Uuid) -> Result<(), DomainError>;
    async fn delete_user(&self, user_id: Uuid) -> Result<(), DomainError>;
}
