use crate::domain::commands::WorkoutTemplateFilterFields;
use crate::domain::entities::WorkoutTemplate;
use crate::domain::entities::WorkoutTemplateSummary;
use crate::domain::errors::DomainError;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::WorkoutTemplateRepository;
use crate::infrastructure::repositories::enums_db::EquipmentDb;
use crate::infrastructure::repositories::enums_db::ExerciseTypeDb;
use crate::infrastructure::repositories::enums_db::MuscleGroupDb;
use crate::infrastructure::repositories::models::ExerciseModel;
use crate::infrastructure::repositories::models::WorkoutTemplateModel;
use crate::infrastructure::repositories::models::WorkoutTemplateRowModel;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

pub struct WorkoutTemplateRepositorySQLX {
    pool: PgPool,
}

impl WorkoutTemplateRepositorySQLX {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn load_exercises(
        &self,
        workout_id: Uuid,
    ) -> Result<Vec<ExerciseModel>, RepositoryError> {
        let exercises = sqlx::query_as!(
            ExerciseModel,
            r#"
            SELECT
                e.id,
                e.user_id,
                e.name,
                e.exercise_type AS "exercise_type: ExerciseTypeDb",
                e.equipment     AS "equipment: EquipmentDb",
                e.muscle_group  AS "muscle_group: MuscleGroupDb",
                e.created_at,
                e.deleted_at
            FROM exercise e
            JOIN workout_exercise we
                ON we.exercise_id = e.id
            WHERE we.workout_id = $1
              AND e.deleted_at IS NULL
            "#,
            workout_id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(exercises)
    }
}

#[async_trait]
impl WorkoutTemplateRepository for WorkoutTemplateRepositorySQLX {
    async fn save(&self, workout: &WorkoutTemplate) -> Result<(), DomainError> {
        let wt: WorkoutTemplateModel = workout.into();

        sqlx::query!(
            r#"
            INSERT INTO workout_template
            (
                id,
                user_id,
                name,
                created_at,
                updated_at,
                deleted_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            wt.id,
            wt.user_id,
            wt.name,
            wt.created_at,
            wt.updated_at,
            wt.deleted_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn read(
        &self,
        fields: WorkoutTemplateFilterFields,
    ) -> Result<Vec<WorkoutTemplateSummary>, DomainError> {
        let mut qb = QueryBuilder::new(
            r#"
            SELECT
                id,
                user_id,
                name,
                created_at,
                updated_at,
                deleted_at
            FROM workout_template
            WHERE user_id =
            "#,
        );

        qb.push_bind(fields.user_id);
        qb.push(" AND deleted_at IS NULL");

        if let Some(name) = fields.name {
            qb.push(" AND name ILIKE ");
            qb.push_bind(format!("%{}%", name));
        }

        let templates: Vec<WorkoutTemplateRowModel> =
            qb.build_query_as().fetch_all(&self.pool).await?;

        let templates: Vec<WorkoutTemplateSummary> = templates
            .into_iter()
            .map(|e| e.try_into())
            .collect::<Result<_, _>>()?;

        Ok(templates)
    }

    async fn find_by_id(&self, workout_id: Uuid) -> Result<WorkoutTemplate, DomainError> {
        let workout = sqlx::query_as!(
            WorkoutTemplateRowModel,
            r#"
            SELECT
                id,
                user_id,
                name,
                created_at,
                updated_at,
                deleted_at
            FROM workout_template
            WHERE id = $1
                AND deleted_at IS NULL;
            "#,
            workout_id
        )
        .fetch_one(&self.pool)
        .await?;

        let exercises = self.load_exercises(workout_id).await?;

        let result = WorkoutTemplateModel::new(workout, exercises);

        let workout_template = result.try_into()?;

        Ok(workout_template)
    }

    async fn update(&self, workout: &WorkoutTemplate) -> Result<(), DomainError> {
        let result = sqlx::query!(
            r#"
            UPDATE workout_template
            SET name = $1,
                updated_at = $2
            WHERE id = $3
                AND deleted_at IS NULL;
            "#,
            workout.name.value(),
            workout.updated_at,
            workout.id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("workout template not found".to_string()).into());
        }

        Ok(())
    }

    async fn soft_delete(&self, workout_id: Uuid) -> Result<(), DomainError> {
        let now = Utc::now();

        let result = sqlx::query!(
            r#"
            UPDATE workout_template
            SET deleted_at = $1,
                updated_at = $2
            WHERE id = $3;
            "#,
            now,
            now,
            workout_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("workout template not found".to_string()).into());
        }

        Ok(())
    }

    async fn delete(&self, workout_id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM workout_template
            WHERE id = $1;
            "#,
            workout_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("workout template not found".to_string()).into());
        }

        Ok(())
    }

    async fn add_exercise(&self, workout_id: Uuid, exercise_id: Uuid) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            INSERT INTO workout_exercise
            (workout_id, exercise_id)
            VALUES($1, $2)
            "#,
            workout_id,
            exercise_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_exercise(
        &self,
        workout_id: Uuid,
        exercise_id: Uuid,
    ) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            DELETE FROM workout_exercise
            WHERE workout_id = $1
                AND exercise_id = $2
            "#,
            workout_id,
            exercise_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
