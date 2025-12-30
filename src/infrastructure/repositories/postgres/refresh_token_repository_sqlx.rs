use crate::{
    domain::{
        entities::refresh_token::RefreshToken, errors::repository_error::RepositoryError,
        repositories::refresh_token_repository::RefreshTokenRepository,
    },
    infrastructure::repositories::models::refresh_token_model::RefreshTokenModel,
};
use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct RefreshTokenRepositorySqlx {
    pub pool: PgPool,
}

impl RefreshTokenRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenRepositorySqlx {
    async fn create(&self, token: RefreshToken) -> Result<(), RepositoryError> {
        let token: RefreshTokenModel = token.into();

        sqlx::query!(
            r#"
            INSERT INTO refresh_token
            (id, user_id, hash, expires_at, revoked, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            token.id,
            token.user_id,
            token.hash,
            token.expires_at,
            token.revoked,
            token.created_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_valid_by_hash(&self, hash: &str) -> Result<RefreshToken, RepositoryError> {
        let result = sqlx::query_as!(
            RefreshTokenModel,
            r#"
            SELECT
              id,
              user_id,
              hash,
              expires_at,
              revoked,
              created_at
            FROM refresh_token
            WHERE hash = $1
              AND revoked = false
              AND expires_at > NOW()
            "#,
            hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.into())
    }

    async fn revoke(&self, token_id: Uuid) -> Result<(), RepositoryError> {
        let result = sqlx::query!(
            r#"
            UPDATE refresh_token
            SET revoked = true
            WHERE id = $1
              AND revoked = false
            "#,
            token_id,
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound("token not found".to_string()));
        }

        Ok(())
    }
}
