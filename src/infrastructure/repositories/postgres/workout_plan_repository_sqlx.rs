use crate::domain::commands::WorkoutPlanFilterFields;
use crate::domain::entities::WorkoutPlan;
use crate::domain::entities::WorkoutPlanRoutineItem;
use crate::domain::entities::WorkoutPlanSummary;
use crate::domain::errors::DomainError;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::WorkoutPlanRepository;
use crate::infrastructure::repositories::enums_db::DayOfWeekDb;
use crate::infrastructure::repositories::enums_db::RoutineItemTypeDb;
use crate::infrastructure::repositories::models::WorkoutPlanRoutineItemRowModel;
use crate::infrastructure::repositories::models::WorkoutPlanRowModel;
use crate::infrastructure::repositories::repo_mapper::to_workout_plan_entity;
use crate::infrastructure::repositories::repo_mapper::to_workout_plan_row_model;
use crate::infrastructure::repositories::repo_mapper::to_workout_plan_summary;
use async_trait::async_trait;
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
    ) -> Result<Vec<WorkoutPlanRoutineItemRowModel>, DomainError> {
        let result = sqlx::query_as::<_, WorkoutPlanRoutineItemRowModel>(
            r#"
            SELECT
                wpri.id,
                wpri.item_type,
                wt.id AS workout_template_id,
                wt.user_id AS workout_template_user_id,
                wt.name AS workout_template_name,
                wpri.day_of_week,
                wpri.position
            FROM workout_plan_routine_item wpri
            LEFT JOIN workout_template wt
                ON wt.id = wpri.workout_template_id
                AND wt.deleted_at IS NULL
            WHERE wpri.workout_plan_id = $1
            ORDER BY wpri.day_of_week NULLS LAST, wpri.position NULLS LAST
            "#,
        )
        .bind(workout_plan_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}

#[async_trait]
impl WorkoutPlanRepository for WorkoutPlanRepositorySQLx {
    async fn save(&self, workout_plan: &WorkoutPlan) -> Result<(), DomainError> {
        let wp = to_workout_plan_row_model(workout_plan);

        sqlx::query(
            r#"
            INSERT INTO workout_plan
            (id, user_id, name, routine_mode, created_at, updated_at, deleted_at)
            VALUES($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(wp.id)
        .bind(wp.user_id)
        .bind(wp.name)
        .bind(wp.routine_mode)
        .bind(wp.created_at)
        .bind(wp.updated_at)
        .bind(wp.deleted_at)
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
                routine_mode,
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
        let result = sqlx::query_as::<_, WorkoutPlanRowModel>(
            r#"
            SELECT
                id,
                user_id,
                name,
                routine_mode,
                created_at,
                updated_at,
                deleted_at
            FROM workout_plan
            WHERE deleted_at IS NULL
                AND id = $1
            "#,
        )
        .bind(workout_plan_id)
        .fetch_one(&self.pool)
        .await?;

        let templates = self.load_workout_templates(workout_plan_id).await?;

        let response = to_workout_plan_entity(result, templates)?;

        Ok(response)
    }

    async fn update(&self, workout_plan: &WorkoutPlan) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            UPDATE workout_plan
            SET name = $1,
                updated_at = $2
            WHERE deleted_at IS NULL
                AND id = $3
            "#,
        )
        .bind(workout_plan.name.value())
        .bind(workout_plan.updated_at)
        .bind(workout_plan.id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("workout plan not found".to_string()).into());
        }

        Ok(())
    }

    async fn soft_delete(&self, workout_plan_id: Uuid) -> Result<(), DomainError> {
        let now = Utc::now();

        let result = sqlx::query(
            r#"
            UPDATE workout_plan
            SET deleted_at = $1,
                updated_at = $1
            WHERE deleted_at IS NULL
                AND id = $2
            "#,
        )
        .bind(now)
        .bind(workout_plan_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("workout plan not found".to_string()).into());
        }

        Ok(())
    }

    async fn delete(&self, workout_plan_id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            DELETE FROM workout_plan
            WHERE id = $1
            "#,
        )
        .bind(workout_plan_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("workout plan not found".to_string()).into());
        }

        Ok(())
    }

    async fn add_routine_item(
        &self,
        routine_item: &WorkoutPlanRoutineItem,
        workout_plan_id: Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO workout_plan_routine_item
            (id, workout_plan_id, workout_template_id, item_type, day_of_week, position)
            VALUES($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(routine_item.id)
        .bind(workout_plan_id)
        .bind(routine_item.workout_template.as_ref().map(|wt| wt.id))
        .bind(RoutineItemTypeDb::from(routine_item.item_type))
        .bind(routine_item.day_of_week.map(DayOfWeekDb::from))
        .bind(routine_item.position.map(|position| position as i32))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_workout_template(
        &self,
        workout_plan_id: Uuid,
        workout_template_id: Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE FROM workout_plan_routine_item
            WHERE workout_plan_id = $1
                AND workout_template_id = $2
            "#,
        )
        .bind(workout_plan_id)
        .bind(workout_template_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_routine_item(
        &self,
        routine_item: &WorkoutPlanRoutineItem,
        workout_plan_id: Uuid,
    ) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            UPDATE workout_plan_routine_item
            SET workout_template_id = $1,
                item_type = $2,
                day_of_week = $3,
                position = $4
            WHERE workout_plan_id = $5
                AND id = $6
            "#,
        )
        .bind(routine_item.workout_template.as_ref().map(|wt| wt.id))
        .bind(RoutineItemTypeDb::from(routine_item.item_type))
        .bind(routine_item.day_of_week.map(DayOfWeekDb::from))
        .bind(routine_item.position.map(|position| position as i32))
        .bind(workout_plan_id)
        .bind(routine_item.id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("routine item not found".to_string()).into());
        }

        Ok(())
    }

    async fn remove_routine_item(
        &self,
        workout_plan_id: Uuid,
        routine_item_id: Uuid,
    ) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            DELETE FROM workout_plan_routine_item
            WHERE workout_plan_id = $1
                AND id = $2
            "#,
        )
        .bind(workout_plan_id)
        .bind(routine_item_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("routine item not found".to_string()).into());
        }

        Ok(())
    }

    async fn find_current_user_plan(&self, user_id: Uuid) -> Result<WorkoutPlan, DomainError> {
        let result = sqlx::query_as::<_, WorkoutPlanRowModel>(
            r#"
            SELECT
              wp.id,
              wp.user_id,
              wp.name,
              wp.routine_mode,
              wp.created_at,
              wp.updated_at,
              wp.deleted_at
            FROM current_workout_plan cwp
            JOIN workout_plan wp
              ON wp.id = cwp.workout_plan_id
            WHERE cwp.user_id = $1
              AND wp.deleted_at IS NULL;
            "#,
        )
        .bind(user_id)
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
