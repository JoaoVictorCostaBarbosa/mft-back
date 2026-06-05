use crate::{
    domain::{
        entities::workout_session::{
            CurrentWorkoutSession, FinishedWorkoutSession, WorkoutSession, WorkoutSessionExercise,
            WorkoutSessionHistoryItem, WorkoutSessionSet, WorkoutSessionWeeklySummaryDay,
        },
        enums::set_type::SetType,
        errors::{domain_error::DomainError, repository_error::RepositoryError},
        repositories::workout_session_repository::WorkoutSessionRepository,
    },
    infrastructure::repositories::{
        enums_db::{set_type_db::SetTypeDb, workout_session_status_db::WorkoutSessionStatusDb},
        models::workout_session_model::{
            CurrentWorkoutSessionRowModel, WorkoutSessionExerciseRowModel,
            WorkoutSessionHistoryRowModel, WorkoutSessionRowModel, WorkoutSessionSetRowModel,
            WorkoutSessionWeeklySummaryRowModel,
        },
    },
};
use axum::async_trait;
use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct WorkoutSessionRepositorySqlx {
    pool: PgPool,
}

impl WorkoutSessionRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WorkoutSessionRepository for WorkoutSessionRepositorySqlx {
    async fn start(&self, session: &WorkoutSession) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO workout_log
            (id, user_id, workout_plan_id, workout_template_id, started_at, finished_at, status, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NULL)
            "#,
        )
        .bind(session.id)
        .bind(session.user_id)
        .bind(session.workout_plan_id)
        .bind(session.workout_template_id)
        .bind(session.started_at)
        .bind(session.finished_at)
        .bind(WorkoutSessionStatusDb::from(session.status))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_current(&self, user_id: Uuid) -> Result<CurrentWorkoutSession, DomainError> {
        let row = sqlx::query_as::<_, CurrentWorkoutSessionRowModel>(
            r#"
            SELECT
                wl.id,
                wt.id AS workout_template_id,
                wt.name AS workout_template_name,
                wl.started_at,
                wl.status
            FROM workout_log wl
            JOIN workout_template wt
                ON wt.id = wl.workout_template_id
            WHERE wl.user_id = $1
                AND wl.status = 'in_progress'
                AND wl.deleted_at IS NULL
                AND wt.deleted_at IS NULL
            ORDER BY wl.started_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(CurrentWorkoutSession {
            id: row.id,
            workout_template_id: row.workout_template_id,
            workout_template_name: row.workout_template_name,
            started_at: row.started_at,
            status: row.status.into(),
        })
    }

    async fn find_by_id(&self, session_id: Uuid) -> Result<WorkoutSession, DomainError> {
        let row = sqlx::query_as::<_, WorkoutSessionRowModel>(
            r#"
            SELECT
                id,
                user_id,
                workout_plan_id,
                workout_template_id,
                started_at,
                finished_at,
                status
            FROM workout_log
            WHERE id = $1
                AND deleted_at IS NULL
            "#,
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(WorkoutSession {
            id: row.id,
            user_id: row.user_id,
            workout_plan_id: row.workout_plan_id,
            workout_template_id: row.workout_template_id,
            started_at: row.started_at,
            finished_at: row.finished_at,
            status: row.status.into(),
        })
    }

    async fn finish(&self, session: &FinishedWorkoutSession) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            UPDATE workout_log
            SET finished_at = $1,
                status = $2
            WHERE id = $3
                AND deleted_at IS NULL
            "#,
        )
        .bind(session.finished_at)
        .bind(WorkoutSessionStatusDb::from(session.status))
        .bind(session.id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("workout session not found".to_string()).into());
        }

        Ok(())
    }

    async fn add_exercise(
        &self,
        session_id: Uuid,
        exercise_id: Uuid,
    ) -> Result<WorkoutSessionExercise, DomainError> {
        let id = Uuid::new_v4();
        let row = sqlx::query_as::<_, WorkoutSessionExerciseRowModel>(
            r#"
            INSERT INTO exercise_log
            (id, workout_log_id, exercise_id)
            VALUES ($1, $2, $3)
            RETURNING id, workout_log_id AS workout_session_id, exercise_id
            "#,
        )
        .bind(id)
        .bind(session_id)
        .bind(exercise_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(WorkoutSessionExercise {
            id: row.id,
            workout_session_id: row.workout_session_id,
            exercise_id: row.exercise_id,
        })
    }

    async fn add_set(
        &self,
        session_id: Uuid,
        exercise_id: Uuid,
        set_type: SetType,
        weight: f32,
        reps: u32,
    ) -> Result<WorkoutSessionSet, DomainError> {
        let exercise_log_id = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT id
            FROM exercise_log
            WHERE workout_log_id = $1
                AND exercise_id = $2
            "#,
        )
        .bind(session_id)
        .bind(exercise_id)
        .fetch_optional(&self.pool)
        .await?;

        let exercise_log_id = match exercise_log_id {
            Some(id) => id,
            None => self.add_exercise(session_id, exercise_id).await?.id,
        };

        let id = Uuid::new_v4();
        let row = sqlx::query_as::<_, WorkoutSessionSetRowModel>(
            r#"
            INSERT INTO set_log
            (id, exercise_log_id, type, weight, reps, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, exercise_log_id AS session_exercise_id, type AS set_type, weight::real AS weight, reps, created_at
            "#,
        )
        .bind(id)
        .bind(exercise_log_id)
        .bind(SetTypeDb::from(set_type))
        .bind(weight)
        .bind(reps as i32)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        Ok(WorkoutSessionSet {
            id: row.id,
            session_exercise_id: row.session_exercise_id,
            set_type: row.set_type.into(),
            weight: row.weight,
            reps: row.reps as u32,
            created_at: row.created_at,
        })
    }

    async fn history(&self, user_id: Uuid) -> Result<Vec<WorkoutSessionHistoryItem>, DomainError> {
        let rows = sqlx::query_as::<_, WorkoutSessionHistoryRowModel>(
            r#"
            SELECT
                wl.id,
                wl.workout_plan_id,
                wt.id AS workout_template_id,
                wt.name AS workout_template_name,
                wl.started_at,
                wl.finished_at,
                wl.status
            FROM workout_log wl
            JOIN workout_template wt
                ON wt.id = wl.workout_template_id
            WHERE wl.user_id = $1
                AND wl.deleted_at IS NULL
            ORDER BY wl.started_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| WorkoutSessionHistoryItem {
                id: row.id,
                workout_plan_id: row.workout_plan_id,
                workout_template_id: row.workout_template_id,
                workout_template_name: row.workout_template_name,
                started_at: row.started_at,
                finished_at: row.finished_at,
                status: row.status.into(),
            })
            .collect())
    }

    async fn weekly_summary(
        &self,
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<WorkoutSessionWeeklySummaryDay>, DomainError> {
        let rows = sqlx::query_as::<_, WorkoutSessionWeeklySummaryRowModel>(
            r#"
            WITH days AS (
                SELECT generate_series($2::date, $3::date, interval '1 day')::date AS date
            )
            SELECT
                days.date,
                lower(trim(to_char(days.date, 'day'))) AS day_of_week,
                wl.id AS session_id
            FROM days
            LEFT JOIN workout_log wl
                ON wl.user_id = $1
                AND wl.deleted_at IS NULL
                AND wl.started_at::date = days.date
            ORDER BY days.date
            "#,
        )
        .bind(user_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| WorkoutSessionWeeklySummaryDay {
                date: row.date,
                day_of_week: row.day_of_week,
                trained: row.session_id.is_some(),
                session_id: row.session_id,
            })
            .collect())
    }

    async fn has_in_progress(&self, user_id: Uuid) -> Result<bool, DomainError> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM workout_log
                WHERE user_id = $1
                    AND status = 'in_progress'
                    AND deleted_at IS NULL
            )
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }
}
