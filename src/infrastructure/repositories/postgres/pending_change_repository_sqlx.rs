use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::PendingChange;
use crate::domain::errors::DomainError;
use crate::domain::repositories::PendingChangesRepository;
use crate::infrastructure::repositories::models::PendingChangeModel;

pub struct PendingChangeRepositorySqlx {
    pool: PgPool,
}

impl PendingChangeRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PendingChangesRepository for PendingChangeRepositorySqlx {
    async fn create_pending_change(
        &self,
        pending_change: PendingChange,
    ) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO pending_change
            (id, user_id, code, limit_date)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(pending_change.id)
        .bind(pending_change.user_id)
        .bind(pending_change.code as i32)
        .bind(pending_change.limit_date)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn get_valid_pending_change_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<PendingChange, DomainError> {
        self.clear_expired_pending_change().await?;

        let result = sqlx::query_as::<_, PendingChangeModel>(
            r#"
            SELECT * FROM pending_change
            WHERE user_id = $1 AND limit_date > NOW()
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(result.to())
    }

    async fn delete_pending_change(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE FROM pending_change
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn clear_expired_pending_change(&self) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE FROM pending_change
            WHERE limit_date < NOW()
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }
}
