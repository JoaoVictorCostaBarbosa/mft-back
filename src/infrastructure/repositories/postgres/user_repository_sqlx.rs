use crate::domain::commands::UserUpdateFields;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::errors::RepositoryError;
use crate::domain::errors::UserError;
use crate::domain::repositories::UserRepository;
use crate::infrastructure::repositories::enums_db::GoalDb;
use crate::infrastructure::repositories::enums_db::RoleDb;
use crate::infrastructure::repositories::models::UserModel;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepositorySQLx {
    pool: PgPool,
}

impl UserRepositorySQLx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositorySQLx {
    async fn create_user(&self, user: &User) -> Result<(), DomainError> {
        let role: RoleDb = user.role.clone().into();

        sqlx::query(
            r#"
            INSERT INTO users
            (id, name, email, password, google_sub, role, url_img, goal, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(user.id)
        .bind(user.name.value())
        .bind(user.email.value())
        .bind(user.password.clone())
        .bind(user.google_sub.clone())
        .bind(role)
        .bind(user.url_img.clone())
        .bind(user.goal.map(GoalDb::from))
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(user.deleted_at)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn create_user_from_pending(
        &self,
        user: &User,
        pending_user_id: Uuid,
    ) -> Result<(), DomainError> {
        let role: RoleDb = user.role.clone().into();

        let mut tx = self.pool.begin().await.map_err(DomainError::from)?;

        sqlx::query(
            r#"
            INSERT INTO users
            (id, name, email, password, google_sub, role, url_img, goal, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(user.id)
        .bind(user.name.value())
        .bind(user.email.value())
        .bind(user.password.clone())
        .bind(user.google_sub.clone())
        .bind(role)
        .bind(user.url_img.clone())
        .bind(user.goal.map(GoalDb::from))
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(user.deleted_at)
        .execute(&mut *tx)
        .await
        .map_err(DomainError::from)?;

        sqlx::query(
            r#"
            DELETE FROM pending_users
            WHERE id = $1
            "#,
        )
        .bind(pending_user_id)
        .execute(&mut *tx)
        .await
        .map_err(DomainError::from)?;

        tx.commit().await.map_err(DomainError::from)?;

        Ok(())
    }

    async fn apply_email_change(
        &self,
        user_id: Uuid,
        email: &str,
        pending_change_id: Uuid,
    ) -> Result<User, DomainError> {
        let now: DateTime<Utc> = Utc::now();

        let mut tx = self.pool.begin().await.map_err(DomainError::from)?;

        let result = sqlx::query_as::<_, UserModel>(
            r#"
            UPDATE users
            SET email = $1,
                updated_at = $2
            WHERE id = $3 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(email)
        .bind(now)
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(DomainError::from)?;

        sqlx::query(
            r#"
            DELETE FROM pending_changes
            WHERE id = $1
            "#,
        )
        .bind(pending_change_id)
        .execute(&mut *tx)
        .await
        .map_err(DomainError::from)?;

        tx.commit().await.map_err(DomainError::from)?;

        let updated_user = result.to_domain()?;

        Ok(updated_user)
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, DomainError> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT * FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(DomainError::from)?;

        let user = result.to_domain()?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT * FROM users
            WHERE email = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(user_model) => {
                let user = user_model.to_domain()?;
                Ok(user)
            }
            Err(e) => Err(DomainError::from(e)),
        }
    }

    async fn get_user_by_google_sub(&self, google_sub: &str) -> Result<User, DomainError> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT * FROM users
            WHERE google_sub = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(google_sub)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(user_model) => Ok(user_model.to_domain()?),
            Err(e) => Err(DomainError::from(e)),
        }
    }

    async fn link_google_sub(
        &self,
        user_id: Uuid,
        google_sub: &str,
        url_img: Option<String>,
    ) -> Result<User, DomainError> {
        let now: DateTime<Utc> = Utc::now();

        let result = sqlx::query_as::<_, UserModel>(
            r#"
            UPDATE users
            SET google_sub = $1,
                url_img = COALESCE(url_img, $2),
                updated_at = $3
            WHERE id = $4
                AND deleted_at IS NULL
                AND (google_sub IS NULL OR google_sub = $1)
            RETURNING *
            "#,
        )
        .bind(google_sub)
        .bind(url_img)
        .bind(now)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(user_model) => Ok(user_model.to_domain()?),
            None => {
                Err(RepositoryError::Conflict("google account already linked".to_string()).into())
            }
        }
    }

    async fn get_all_users(&self) -> Result<Vec<User>, DomainError> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT * FROM users
            WHERE deleted_at IS NULL
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(DomainError::from)?;

        let users = result
            .iter()
            .map(|user| user.to_domain())
            .collect::<Result<Vec<_>, UserError>>()?;

        Ok(users)
    }

    async fn update_user(
        &self,
        user: UserUpdateFields,
        user_id: Uuid,
    ) -> Result<User, DomainError> {
        let now: DateTime<Utc> = Utc::now();

        let result = sqlx::query_as::<_, UserModel>(
            r#"
            UPDATE users
            SET name = COALESCE($1, name),
                email = COALESCE($2, email),
                password = COALESCE($3, password),
                url_img = COALESCE($4, url_img),
                goal = COALESCE($5, goal),
                updated_at = $6
            WHERE id = $7 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(user.name)
        .bind(user.email)
        .bind(user.password)
        .bind(user.url_img)
        .bind(user.goal.map(GoalDb::from))
        .bind(now)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(DomainError::from)?;

        let updated_user = result.to_domain()?;

        Ok(updated_user)
    }

    async fn soft_delete_user(&self, user_id: Uuid) -> Result<(), DomainError> {
        let now: DateTime<Utc> = Utc::now();

        sqlx::query(
            r#"
            UPDATE users
            SET deleted_at = $1,
                updated_at = $2
            WHERE id = $3 AND deleted_at IS NULL
            "#,
        )
        .bind(now)
        .bind(now)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn restore_user(&self, user_id: Uuid) -> Result<(), DomainError> {
        let now: DateTime<Utc> = Utc::now();

        sqlx::query(
            r#"
            UPDATE users
            SET deleted_at = NULL,
                updated_at = $1
            WHERE id = $2
            "#,
        )
        .bind(now)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        Ok(())
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(DomainError::from)?;

        if result.rows_affected() == 0 {
            return Err(DomainError::Repository(RepositoryError::NotFound(
                "User not found".to_string(),
            )));
        }

        Ok(())
    }
}
