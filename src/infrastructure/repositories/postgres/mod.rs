use crate::{
    application::interfaces::{
        pending_change_repository::PendingChangesRepository,
        pending_user_repository::PendingUserRepository,
    },
    domain::repositories::user_repository::UserRepository,
    infrastructure::repositories::postgres::{
        pending_change_repository_sqlx::PendingChangeRepositorySqlx, pending_user_repository_sqlx::PendingUserRepositorySqlx, user_repository_sqlx::UserRepositorySQLx
    },
};
use sqlx::PgPool;
use std::sync::Arc;

pub mod error;
pub mod pending_change_repository_sqlx;
pub mod pending_user_repository_sqlx;
pub mod user_repository_sqlx;

pub struct RepositoryBundle {
    pub user_repo: Arc<dyn UserRepository>,
    pub pending_user_repo: Arc<dyn PendingUserRepository>,
    pub pending_change_repo: Arc<dyn PendingChangesRepository>,
}

impl RepositoryBundle {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_repo: Arc::new(UserRepositorySQLx::new(pool.clone())),
            pending_user_repo: Arc::new(PendingUserRepositorySqlx::new(pool.clone())),
            pending_change_repo: Arc::new(PendingChangeRepositorySqlx::new(pool.clone())),
        }
    }
}
