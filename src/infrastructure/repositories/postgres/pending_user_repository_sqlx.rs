use crate::domain::entities::PendingUser;
use crate::domain::errors::DomainError;
use crate::domain::repositories::PendingUserRepository;
use crate::infrastructure::repositories::models::PendingUserModel;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PendingUserRepositorySqlx {
    pool: PgPool,
}

impl PendingUserRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PendingUserRepository for PendingUserRepositorySqlx {
    async fn create_pending_user(&self, pending_user: PendingUser) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO pending_users
            (id, name, email, password, code, limit_date)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(pending_user.id)
        .bind(pending_user.name)
        .bind(pending_user.email)
        .bind(pending_user.password)
        .bind(pending_user.code as i32)
        .bind(pending_user.limit_date)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn get_valid_pending_user_by_email(
        &self,
        email: &str,
    ) -> Result<PendingUser, DomainError> {
        self.clear_expired_pending_user().await?;

        let pending_user = sqlx::query_as::<_, PendingUserModel>(
            r#"
            SELECT * FROM pending_users
            WHERE email = $1 AND limit_date > NOW()
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(pending_user.to())
    }

    async fn delete_pending_user(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE FROM pending_users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn clear_expired_pending_user(&self) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE FROM pending_users
            WHERE limit_date < NOW()
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }
}
