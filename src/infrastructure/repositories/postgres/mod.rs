use crate::{
    application::interfaces::{
        pending_change_repository::PendingChangesRepository,
        pending_user_repository::PendingUserRepository,
    },
    domain::repositories::{
        exercise_repository::ExerciseRepository, measurement_repository::MeasurementRepository,
        refresh_token_repository::RefreshTokenRepository, user_repository::UserRepository,
    },
    infrastructure::repositories::postgres::{
        exercise_repository_sqlx::ExerciseRepositorySqlx,
        measurement_repository_sqlx::MeasurementRepositorySqlx,
        pending_change_repository_sqlx::PendingChangeRepositorySqlx,
        pending_user_repository_sqlx::PendingUserRepositorySqlx,
        refresh_token_repository_sqlx::RefreshTokenRepositorySqlx,
        user_repository_sqlx::UserRepositorySQLx,
    },
};
use sqlx::PgPool;
use std::sync::Arc;

pub mod error;
pub mod exercise_repository_sqlx;
pub mod measurement_repository_sqlx;
pub mod pending_change_repository_sqlx;
pub mod pending_user_repository_sqlx;
pub mod refresh_token_repository_sqlx;
pub mod user_repository_sqlx;

pub struct RepositoryBundle {
    pub user_repo: Arc<dyn UserRepository>,
    pub pending_user_repo: Arc<dyn PendingUserRepository>,
    pub refresh_token_repo: Arc<dyn RefreshTokenRepository>,
    pub pending_change_repo: Arc<dyn PendingChangesRepository>,
    pub measurement_repo: Arc<dyn MeasurementRepository>,
    pub exercise_repo: Arc<dyn ExerciseRepository>,
}

impl RepositoryBundle {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_repo: Arc::new(UserRepositorySQLx::new(pool.clone())),
            pending_user_repo: Arc::new(PendingUserRepositorySqlx::new(pool.clone())),
            refresh_token_repo: Arc::new(RefreshTokenRepositorySqlx::new(pool.clone())),
            pending_change_repo: Arc::new(PendingChangeRepositorySqlx::new(pool.clone())),
            measurement_repo: Arc::new(MeasurementRepositorySqlx::new(pool.clone())),
            exercise_repo: Arc::new(ExerciseRepositorySqlx::new(pool.clone())),
        }
    }
}
