use crate::application::ports::ExerciseQueries;
use crate::application::read_models::ExerciseLastPerformance;
use crate::application::read_models::ExercisePersonalRecord;
use crate::application::read_models::ExerciseLastPerformanceSet;
use crate::domain::commands::ExerciseFilterFields;
use crate::domain::commands::ExercisePaginationFields;
use crate::domain::commands::ExerciseUpdateFields;
use crate::domain::entities::Exercise;
use crate::domain::entities::Paginated;
use crate::domain::errors::DomainError;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::ExerciseRepository;
use crate::infrastructure::repositories::enums_db::EquipmentDb;
use crate::infrastructure::repositories::enums_db::ExerciseTypeDb;
use crate::infrastructure::repositories::enums_db::MuscleGroupDb;
use crate::infrastructure::repositories::models::ExerciseModel;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct ExerciseLastSessionRow {
    exercise_id: Uuid,
    last_session_id: Uuid,
    performed_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct ExercisePersonalRecordRow {
    exercise_id: Uuid,
    exercise_name: String,
    max_weight: f32,
    reps: i32,
    achieved_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct ExerciseLastPerformanceSetRow {
    exercise_id: Uuid,
    session_id: Uuid,
    set_type: crate::infrastructure::repositories::enums_db::SetTypeDb,
    weight: f32,
    reps: i32,
    order: i64,
}

pub struct ExerciseRepositorySqlx {
    pool: PgPool,
}

impl ExerciseRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExerciseRepository for ExerciseRepositorySqlx {
    async fn create_exercise(&self, exercise: &Exercise) -> Result<(), DomainError> {
        let exercise: ExerciseModel = exercise.into();

        let result = sqlx::query(
            r#"
            INSERT INTO exercise
            (id, user_id, name, exercise_type, equipment, muscle_group, created_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(exercise.id)
        .bind(exercise.user_id)
        .bind(exercise.name)
        .bind(exercise.exercise_type)
        .bind(exercise.equipment)
        .bind(exercise.muscle_group)
        .bind(exercise.created_at)
        .bind(exercise.deleted_at)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() != 1 {
            return Err(RepositoryError::Unexpected(format!(
                "Expected 1 row affected, got {}",
                result.rows_affected()
            ))
            .into());
        }

        Ok(())
    }

    async fn get_exercises(
        &self,
        fields: ExerciseFilterFields,
    ) -> Result<Paginated<Exercise>, DomainError> {
        let mut qb = QueryBuilder::new(
            r#"
                SELECT
                    id,
                    user_id,
                    name,
                    exercise_type,
                    equipment,
                    muscle_group,
                    created_at,
                    deleted_at
                FROM exercise
                WHERE deleted_at IS NULL
                "#,
        );

        push_exercise_filters(&mut qb, &fields);

        qb.push(" ORDER BY name ASC, id ASC");

        if let Some(pagination) = fields.pagination {
            qb.push(" LIMIT ");
            qb.push_bind(pagination.per_page as i64);
            qb.push(" OFFSET ");
            qb.push_bind(((pagination.page - 1) * pagination.per_page) as i64);
        }

        let models: Vec<ExerciseModel> = qb.build_query_as().fetch_all(&self.pool).await?;

        let exercises = models
            .into_iter()
            .map(Exercise::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let mut count_qb = QueryBuilder::new(
            r#"
                SELECT COUNT(*)
                FROM exercise
                WHERE deleted_at IS NULL
                "#,
        );

        push_exercise_filters(&mut count_qb, &fields);

        let total_items: i64 = count_qb.build_query_scalar().fetch_one(&self.pool).await?;
        let pagination = fields.pagination.unwrap_or(ExercisePaginationFields {
            page: 1,
            per_page: total_items.max(1) as u32,
        });

        Ok(Paginated::new(
            exercises,
            total_items,
            pagination.per_page,
            pagination.page,
        ))
    }

    async fn update_exercise(
        &self,
        fields: ExerciseUpdateFields,
        user_id: Option<Uuid>,
    ) -> Result<(), DomainError> {
        let mut qb = QueryBuilder::new("UPDATE exercise SET ");
        let mut separated = false;

        if let Some(name) = fields.name {
            if separated {
                qb.push(", ");
            }
            qb.push("name = ");
            qb.push_bind(name);
            separated = true;
        }

        if let Some(exercise_type) = fields.exercise_type {
            if separated {
                qb.push(", ");
            }
            qb.push("exercise_type = ");
            qb.push_bind(ExerciseTypeDb::from(exercise_type));
            separated = true;
        }

        if let Some(equipment) = fields.equipment {
            if separated {
                qb.push(", ");
            }
            qb.push("equipment = ");
            qb.push_bind(EquipmentDb::from(equipment));
            separated = true;
        }

        if let Some(muscle_group) = fields.muscle_group {
            if separated {
                qb.push(", ");
            }
            qb.push("muscle_group = ");
            qb.push_bind(MuscleGroupDb::from(muscle_group));
            separated = true;
        }

        if !separated {
            return Err(RepositoryError::DbError("invalid update".to_string()).into());
        }

        qb.push(" WHERE deleted_at IS NULL AND id = ");
        qb.push_bind(fields.id);

        if let Some(user_id) = user_id {
            qb.push(" AND user_id = ");
            qb.push_bind(user_id);
        }

        let result = qb.build().execute(&self.pool).await?;

        if result.rows_affected() != 1 {
            return Err(RepositoryError::NotFound("exercise not found".to_string()).into());
        }

        Ok(())
    }

    async fn soft_delete_exercise(&self, id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            UPDATE exercise
            SET deleted_at = $1
            WHERE id = $2
                AND user_id = $3
                AND deleted_at IS NULL
            "#,
        )
        .bind(Utc::now())
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() != 1 {
            return Err(RepositoryError::NotFound("exercise not found".to_string()).into());
        }

        Ok(())
    }

    async fn delete_exercise(&self, id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            DELETE FROM exercise
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() != 1 {
            return Err(RepositoryError::NotFound("exercise not found".to_string()).into());
        }

        Ok(())
    }

    async fn read_by_id(&self, exercise_id: Uuid) -> Result<Exercise, DomainError> {
        let result = sqlx::query_as!(
            ExerciseModel,
            r#"
            SELECT
                id,
                user_id,
                name,
                exercise_type AS "exercise_type: ExerciseTypeDb",
                equipment     AS "equipment: EquipmentDb",
                muscle_group  AS "muscle_group: MuscleGroupDb",
                created_at,
                deleted_at
            FROM exercise
            WHERE deleted_at IS NULL
                AND id = $1
            "#,
            exercise_id,
        )
        .fetch_one(&self.pool)
        .await?;

        let exercise: Exercise = result.try_into()?;

        Ok(exercise)
    }
}

#[async_trait]
impl ExerciseQueries for ExerciseRepositorySqlx {
    async fn find_last_performances(
        &self,
        user_id: Uuid,
        exercise_ids: Vec<Uuid>,
    ) -> Result<Vec<ExerciseLastPerformance>, DomainError> {
        let last_sessions = sqlx::query_as::<_, ExerciseLastSessionRow>(
            r#"
            SELECT DISTINCT ON (el.exercise_id)
                el.exercise_id,
                wl.id AS last_session_id,
                wl.finished_at AS performed_at
            FROM exercise_log el
            JOIN workout_log wl
                ON wl.id = el.workout_log_id
            JOIN exercise e
                ON e.id = el.exercise_id
            WHERE wl.user_id = $1
                AND wl.status = 'finished'
                AND wl.finished_at IS NOT NULL
                AND wl.deleted_at IS NULL
                AND e.deleted_at IS NULL
                AND (e.user_id IS NULL OR e.user_id = $1)
                AND el.exercise_id = ANY($2)
            ORDER BY el.exercise_id, wl.finished_at DESC, wl.started_at DESC
            "#,
        )
        .bind(user_id)
        .bind(&exercise_ids)
        .fetch_all(&self.pool)
        .await?;

        if last_sessions.is_empty() {
            return Ok(Vec::new());
        }

        let session_ids = last_sessions
            .iter()
            .map(|session| session.last_session_id)
            .collect::<Vec<_>>();

        let set_rows = sqlx::query_as::<_, ExerciseLastPerformanceSetRow>(
            r#"
            SELECT
                el.exercise_id,
                el.workout_log_id AS session_id,
                sl.type AS set_type,
                sl.weight::real AS weight,
                sl.reps,
                ROW_NUMBER() OVER (
                    PARTITION BY el.exercise_id, el.workout_log_id
                    ORDER BY sl.created_at ASC, sl.id ASC
                ) AS "order"
            FROM exercise_log el
            JOIN set_log sl
                ON sl.exercise_log_id = el.id
            WHERE el.workout_log_id = ANY($1)
                AND el.exercise_id = ANY($2)
            ORDER BY el.exercise_id, sl.created_at ASC, sl.id ASC
            "#,
        )
        .bind(&session_ids)
        .bind(&exercise_ids)
        .fetch_all(&self.pool)
        .await?;

        let mut performances = last_sessions
            .into_iter()
            .map(|session| ExerciseLastPerformance {
                exercise_id: session.exercise_id,
                last_session_id: session.last_session_id,
                performed_at: session.performed_at,
                sets: Vec::new(),
            })
            .collect::<Vec<_>>();

        for set_row in set_rows {
            if let Some(performance) = performances.iter_mut().find(|performance| {
                performance.exercise_id == set_row.exercise_id
                    && performance.last_session_id == set_row.session_id
            }) {
                performance.sets.push(ExerciseLastPerformanceSet {
                    set_type: set_row.set_type.into(),
                    weight: set_row.weight,
                    reps: set_row.reps as u32,
                    order: set_row.order as u32,
                });
            }
        }

        Ok(performances)
    }

    async fn find_personal_records(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ExercisePersonalRecord>, DomainError> {
        let rows = sqlx::query_as::<_, ExercisePersonalRecordRow>(
            r#"
            SELECT
                exercise_id,
                exercise_name,
                max_weight,
                reps,
                achieved_at
            FROM (
                SELECT DISTINCT ON (el.exercise_id)
                    el.exercise_id,
                    e.name AS exercise_name,
                    sl.weight::real AS max_weight,
                    sl.reps,
                    sl.created_at AS achieved_at
                FROM set_log sl
                JOIN exercise_log el ON el.id = sl.exercise_log_id
                JOIN workout_log wl ON wl.id = el.workout_log_id
                JOIN exercise e ON e.id = el.exercise_id
                WHERE wl.user_id = $1
                    AND wl.deleted_at IS NULL
                    AND e.deleted_at IS NULL
                ORDER BY el.exercise_id, sl.weight DESC, sl.created_at ASC
            ) records
            ORDER BY max_weight DESC, exercise_name ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| ExercisePersonalRecord {
                exercise_id: row.exercise_id,
                exercise_name: row.exercise_name,
                max_weight: row.max_weight,
                reps: row.reps as u32,
                achieved_at: row.achieved_at,
            })
            .collect())
    }
}

fn push_exercise_filters<'args>(
    qb: &mut QueryBuilder<'args, sqlx::Postgres>,
    fields: &ExerciseFilterFields,
) {
    match fields.user_id {
        Some(user_id) => {
            qb.push(" AND (user_id IS NULL OR user_id = ");
            qb.push_bind(user_id);
            qb.push(")");
        }
        None => {
            qb.push(" AND user_id IS NULL");
        }
    }

    if let Some(id) = fields.id {
        qb.push(" AND id = ");
        qb.push_bind(id);
    }

    if let Some(name) = &fields.name {
        let escaped = name.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_");
        qb.push(" AND name ILIKE ");
        qb.push_bind(format!("%{escaped}%"));
    }

    if let Some(exercise_type) = fields.exercise_type {
        qb.push(" AND exercise_type = ");
        qb.push_bind(ExerciseTypeDb::from(exercise_type));
    }

    if let Some(equipment) = fields.equipment {
        qb.push(" AND equipment = ");
        qb.push_bind(EquipmentDb::from(equipment));
    }

    if let Some(muscle_group) = fields.muscle_group {
        qb.push(" AND muscle_group = ");
        qb.push_bind(MuscleGroupDb::from(muscle_group));
    }
}
