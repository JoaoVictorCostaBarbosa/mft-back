use crate::{
    domain::{
        commands::workout_plan_command::WorkoutPlanFilterFields,
        entities::workout_plan::{WorkoutPlan, WorkoutPlanSummary},
        errors::{domain_error::DomainError, repository_error::RepositoryError},
        repositories::workout_plan_repository::WorkoutPlanRepository,
    },
    infrastructure::repositories::{
        models::{
            workout_plan_model::WorkoutPlanRowModel,
            workout_template_model::WorkoutTemplateRowModel,
        },
        repo_mapper::wp_repo_mapper::{
            to_workout_plan_entity, to_workout_plan_row_model, to_workout_plan_summary,
        },
    },
};
use axum::async_trait;
use chrono::Utc;
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

pub struct WorkoutPlanRepositorySQLx {
    pool: PgPool,
}

impl WorkoutPlanRepositorySQLx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn load_workout_templates(
        &self,
        workout_plan_id: Uuid,
    ) -> Result<Vec<WorkoutTemplateRowModel>, DomainError> {
        let result = sqlx::query_as!(
            WorkoutTemplateRowModel,
            r#"
            SELECT
                wt.id,
                wt.user_id,
                wt.name,
                wt.created_at,
                wt.updated_at,
                wt.deleted_at
            FROM workout_template wt
            JOIN workout_template_in_plan wtp
                ON wt.id = wtp.workout_template_id
            WHERE wtp.workout_plan_id = $1
                AND wt.deleted_at IS NULL
            "#,
            workout_plan_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}

#[async_trait]
impl WorkoutPlanRepository for WorkoutPlanRepositorySQLx {
    async fn save(&self, workout_plan: &WorkoutPlan) -> Result<(), DomainError> {
        let wp = to_workout_plan_row_model(workout_plan);

        sqlx::query!(
            r#"
            INSERT INTO workout_plan
            (id, user_id, name, created_at, updated_at, deleted_at)
            VALUES($1, $2, $3, $4, $5, $6)
            "#,
            wp.id,
            wp.user_id,
            wp.name,
            wp.created_at,
            wp.updated_at,
            wp.deleted_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn read_summary(
        &self,
        filter: WorkoutPlanFilterFields,
    ) -> Result<Vec<WorkoutPlanSummary>, DomainError> {
        let mut qb = QueryBuilder::new(
            r#"
            SELECT
                id,
                user_id,
                name,
                created_at,
                updated_at,
                deleted_at
            FROM workout_plan
            WHERE deleted_at IS NULL
                AND user_id =
            "#,
        );

        qb.push_bind(filter.user_id);

        if let Some(name) = filter.name {
            qb.push("AND name ILIKE ");
            qb.push_bind(format!("%{}%", name));
        }

        let workout_plans: Vec<WorkoutPlanRowModel> =
            qb.build_query_as().fetch_all(&self.pool).await?;

        let workout_plans: Vec<WorkoutPlanSummary> = workout_plans
            .into_iter()
            .map(|wp| to_workout_plan_summary(wp))
            .collect::<Result<_, _>>()?;

        Ok(workout_plans)
    }

    async fn find_by_id(&self, workout_plan_id: Uuid) -> Result<WorkoutPlan, DomainError> {
        let result = sqlx::query_as!(
            WorkoutPlanRowModel,
            r#"
            SELECT
                id,
                user_id,
                name,
                created_at,
                updated_at,
                deleted_at
            FROM workout_plan
            WHERE deleted_at IS NULL
                AND id = $1
            "#,
            workout_plan_id
        )
        .fetch_one(&self.pool)
        .await?;

        let templates = self.load_workout_templates(workout_plan_id).await?;

        let response = to_workout_plan_entity(result, templates)?;

        Ok(response)
    }

    async fn update(&self, workout_plan: &WorkoutPlan) -> Result<(), DomainError> {
        let result = sqlx::query!(
            r#"
            UPDATE workout_plan
            SET name = $1,
                updated_at = $2
            WHERE deleted_at IS NULL
                AND id = $3
            "#,
            workout_plan.name.value(),
            workout_plan.updated_at,
            workout_plan.id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("workout plan not found".to_string()).into());
        }

        Ok(())
    }

    async fn soft_delete(&self, workout_plan_id: Uuid) -> Result<(), DomainError> {
        let now = Utc::now();

        let result = sqlx::query!(
            r#"
            UPDATE workout_plan
            SET deleted_at = $1,
                updated_at = $1
            WHERE deleted_at IS NULL
                AND id = $2
            "#,
            now,
            workout_plan_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("workout plan not found".to_string()).into());
        }

        Ok(())
    }

    async fn delete(&self, workout_plan_id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM workout_plan
            WHERE id = $1
            "#,
            workout_plan_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("workout plan not found".to_string()).into());
        }

        Ok(())
    }

    async fn add_workout_template(
        &self,
        workout_plan_id: Uuid,
        workout_template_id: Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            INSERT INTO workout_template_in_plan
            (workout_plan_id, workout_template_id)
            VALUES($1, $2)
            "#,
            workout_plan_id,
            workout_template_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_workout_template(
        &self,
        workout_plan_id: Uuid,
        workout_template_id: Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            DELETE FROM workout_template_in_plan
            WHERE workout_plan_id = $1
                AND workout_template_id = $2
            "#,
            workout_plan_id,
            workout_template_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_current_user_plan(&self, user_id: Uuid) -> Result<WorkoutPlan, DomainError> {
        let result = sqlx::query_as!(
            WorkoutPlanRowModel,
            r#"
            SELECT
              wp.id,
              wp.user_id,
              wp.name,
              wp.created_at,
              wp.updated_at,
              wp.deleted_at
            FROM current_workout_plan cwp
            JOIN workout_plan wp
              ON wp.id = cwp.workout_plan_id
            WHERE cwp.user_id = $1
              AND wp.deleted_at IS NULL;
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        let wts = self.load_workout_templates(result.id).await?;

        Ok(to_workout_plan_entity(result, wts)?)
    }

    async fn set_current(&self, user_id: Uuid, wp_id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO current_workout_plan
            (user_id, workout_plan_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id)
            DO UPDATE SET workout_plan_id = EXCLUDED.workout_plan_id
            "#,
        )
        .bind(user_id)
        .bind(wp_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
