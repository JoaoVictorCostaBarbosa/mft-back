use crate::domain::entities::RefreshToken;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::RefreshTokenRepository;
use crate::infrastructure::repositories::models::RefreshTokenModel;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct RefreshTokenRepositorySqlx {
    pool: PgPool,
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
    async fn rotate(
        &self,
        revoked_id: Uuid,
        new_token: RefreshToken,
    ) -> Result<(), RepositoryError> {
        let new_token: RefreshTokenModel = new_token.into();

        let mut tx = self.pool.begin().await?;

        let result = sqlx::query!(
            r#"
            UPDATE refresh_token
            SET revoked = true
            WHERE id = $1
              AND revoked = false
            "#,
            revoked_id,
        )
        .execute(&mut *tx)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound("token not found".to_string()));
        }

        sqlx::query!(
            r#"
            INSERT INTO refresh_token
            (id, user_id, hash, expires_at, revoked, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            new_token.id,
            new_token.user_id,
            new_token.hash,
            new_token.expires_at,
            new_token.revoked,
            new_token.created_at,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
