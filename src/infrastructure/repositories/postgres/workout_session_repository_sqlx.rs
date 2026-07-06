use crate::application::ports::WorkoutSessionQueries;
use crate::application::read_models::CurrentWorkoutSession;
use crate::application::read_models::WorkoutSessionDetailedExercise;
use crate::application::read_models::WorkoutSessionExerciseDetails;
use crate::application::read_models::WorkoutSessionHistoryItem;
use crate::application::read_models::WorkoutSessionWeeklySummary;
use crate::application::read_models::WorkoutSessionWeeklySummaryDay;
use crate::domain::entities::FinishedWorkoutSession;
use crate::domain::entities::WorkoutSession;
use crate::domain::entities::WorkoutSessionExercise;
use crate::domain::entities::WorkoutSessionSet;
use crate::domain::enums::SetType;
use crate::domain::errors::DomainError;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::WorkoutSessionRepository;
use crate::infrastructure::repositories::enums_db::SetTypeDb;
use crate::infrastructure::repositories::enums_db::WorkoutSessionStatusDb;
use crate::infrastructure::repositories::models::CurrentWorkoutSessionRowModel;
use crate::infrastructure::repositories::models::WorkoutSessionDetailedExerciseRowModel;
use crate::infrastructure::repositories::models::WorkoutSessionExerciseRowModel;
use crate::infrastructure::repositories::models::WorkoutSessionHistoryRowModel;
use crate::infrastructure::repositories::models::WorkoutSessionRowModel;
use crate::infrastructure::repositories::models::WorkoutSessionSetRowModel;
use crate::infrastructure::repositories::models::WorkoutSessionWeeklySummaryRowModel;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
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

    async fn cancel(&self, session_id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            UPDATE workout_log
            SET status = 'cancelled',
                deleted_at = COALESCE(deleted_at, NOW())
            WHERE id = $1
                AND deleted_at IS NULL
            "#,
        )
        .bind(session_id)
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
        client_operation_id: Option<Uuid>,
    ) -> Result<WorkoutSessionExercise, DomainError> {
        if let Some(client_operation_id) = client_operation_id {
            let existing = sqlx::query_as::<_, WorkoutSessionExerciseRowModel>(
                r#"
                SELECT
                    id,
                    workout_log_id AS workout_session_id,
                    client_operation_id,
                    exercise_id,
                    position AS "order"
                FROM exercise_log
                WHERE workout_log_id = $1
                    AND client_operation_id = $2
                "#,
            )
            .bind(session_id)
            .bind(client_operation_id)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(row) = existing {
                return Ok(WorkoutSessionExercise {
                    id: row.id,
                    workout_session_id: row.workout_session_id,
                    client_operation_id: row.client_operation_id,
                    exercise_id: row.exercise_id,
                    order: row.order,
                });
            }
        }

        let id = Uuid::new_v4();
        let row = sqlx::query_as::<_, WorkoutSessionExerciseRowModel>(
            r#"
            INSERT INTO exercise_log
            (id, workout_log_id, client_operation_id, exercise_id, position)
            VALUES (
                $1,
                $2,
                $3,
                $4,
                COALESCE((SELECT MAX(position) + 1 FROM exercise_log WHERE workout_log_id = $2), 1)
            )
            RETURNING
                id,
                workout_log_id AS workout_session_id,
                client_operation_id,
                exercise_id,
                position AS "order"
            "#,
        )
        .bind(id)
        .bind(session_id)
        .bind(client_operation_id)
        .bind(exercise_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(WorkoutSessionExercise {
            id: row.id,
            workout_session_id: row.workout_session_id,
            client_operation_id: row.client_operation_id,
            exercise_id: row.exercise_id,
            order: row.order,
        })
    }

    async fn add_set(
        &self,
        session_id: Uuid,
        exercise_id: Uuid,
        set_type: SetType,
        weight: f32,
        reps: u32,
        client_operation_id: Option<Uuid>,
        completed_at: Option<DateTime<Utc>>,
    ) -> Result<WorkoutSessionSet, DomainError> {
        if let Some(client_operation_id) = client_operation_id {
            let existing = sqlx::query_as::<_, WorkoutSessionSetRowModel>(
                r#"
                SELECT
                    sl.id,
                    sl.exercise_log_id AS session_exercise_id,
                    sl.client_operation_id,
                    sl.type AS set_type,
                    sl.weight::real AS weight,
                    sl.reps,
                    sl.created_at
                FROM set_log sl
                JOIN exercise_log el
                    ON el.id = sl.exercise_log_id
                WHERE el.workout_log_id = $1
                    AND sl.client_operation_id = $2
                "#,
            )
            .bind(session_id)
            .bind(client_operation_id)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(row) = existing {
                return Ok(WorkoutSessionSet {
                    id: row.id,
                    session_exercise_id: row.session_exercise_id,
                    client_operation_id: row.client_operation_id,
                    set_type: row.set_type.into(),
                    weight: row.weight,
                    reps: row.reps as u32,
                    created_at: row.created_at,
                });
            }
        }

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
            None => self.add_exercise(session_id, exercise_id, None).await?.id,
        };

        let id = Uuid::new_v4();
        let row = sqlx::query_as::<_, WorkoutSessionSetRowModel>(
            r#"
            INSERT INTO set_log
            (id, exercise_log_id, client_operation_id, type, weight, reps, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING
                id,
                exercise_log_id AS session_exercise_id,
                client_operation_id,
                type AS set_type,
                weight::real AS weight,
                reps,
                created_at
            "#,
        )
        .bind(id)
        .bind(exercise_log_id)
        .bind(client_operation_id)
        .bind(SetTypeDb::from(set_type))
        .bind(weight)
        .bind(reps as i32)
        .bind(completed_at.unwrap_or_else(Utc::now))
        .fetch_one(&self.pool)
        .await?;

        Ok(WorkoutSessionSet {
            id: row.id,
            session_exercise_id: row.session_exercise_id,
            client_operation_id: row.client_operation_id,
            set_type: row.set_type.into(),
            weight: row.weight,
            reps: row.reps as u32,
            created_at: row.created_at,
        })
    }

    async fn update_set(
        &self,
        session_id: Uuid,
        set_id: Uuid,
        set_type: SetType,
        weight: f32,
        reps: u32,
    ) -> Result<WorkoutSessionSet, DomainError> {
        let row = sqlx::query_as::<_, WorkoutSessionSetRowModel>(
            r#"
            UPDATE set_log sl
            SET type = $3,
                weight = $4,
                reps = $5
            FROM exercise_log el
            WHERE sl.id = $2
                AND sl.exercise_log_id = el.id
                AND el.workout_log_id = $1
            RETURNING
                sl.id,
                sl.exercise_log_id AS session_exercise_id,
                sl.client_operation_id,
                sl.type AS set_type,
                sl.weight::real AS weight,
                sl.reps,
                sl.created_at
            "#,
        )
        .bind(session_id)
        .bind(set_id)
        .bind(SetTypeDb::from(set_type))
        .bind(weight)
        .bind(reps as i32)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(WorkoutSessionSet {
                id: row.id,
                session_exercise_id: row.session_exercise_id,
                client_operation_id: row.client_operation_id,
                set_type: row.set_type.into(),
                weight: row.weight,
                reps: row.reps as u32,
                created_at: row.created_at,
            }),
            None => Err(RepositoryError::NotFound("set not found".to_string()).into()),
        }
    }

    async fn delete_set(&self, session_id: Uuid, set_id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            DELETE FROM set_log sl
            USING exercise_log el
            WHERE sl.id = $2
                AND sl.exercise_log_id = el.id
                AND el.workout_log_id = $1
            "#,
        )
        .bind(session_id)
        .bind(set_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("set not found".to_string()).into());
        }

        Ok(())
    }

    async fn reorder_exercises(
        &self,
        session_id: Uuid,
        ordered_session_exercise_ids: Vec<Uuid>,
    ) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await?;

        for (index, session_exercise_id) in ordered_session_exercise_ids.iter().enumerate() {
            let result = sqlx::query(
                r#"
                UPDATE exercise_log
                SET position = $3
                WHERE workout_log_id = $1
                    AND id = $2
                "#,
            )
            .bind(session_id)
            .bind(session_exercise_id)
            .bind((index + 1) as i32)
            .execute(&mut *tx)
            .await?;

            if result.rows_affected() < 1 {
                return Err(
                    RepositoryError::NotFound("session exercise not found".to_string()).into(),
                );
            }
        }

        tx.commit().await?;
        Ok(())
    }

    async fn find_session_exercise_ids(&self, session_id: Uuid) -> Result<Vec<Uuid>, DomainError> {
        let rows = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT id
            FROM exercise_log
            WHERE workout_log_id = $1
            ORDER BY position ASC
            "#,
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn remove_exercise(
        &self,
        session_id: Uuid,
        session_exercise_id: Uuid,
    ) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"
            DELETE FROM set_log
            WHERE exercise_log_id = $1
            "#,
        )
        .bind(session_exercise_id)
        .execute(&mut *tx)
        .await?;

        let result = sqlx::query(
            r#"
            DELETE FROM exercise_log
            WHERE workout_log_id = $1
                AND id = $2
            "#,
        )
        .bind(session_id)
        .bind(session_exercise_id)
        .execute(&mut *tx)
        .await?;

        if result.rows_affected() < 1 {
            return Err(RepositoryError::NotFound("session exercise not found".to_string()).into());
        }

        sqlx::query(
            r#"
            WITH ordered AS (
                SELECT
                    id,
                    ROW_NUMBER() OVER (ORDER BY position ASC) AS new_position
                FROM exercise_log
                WHERE workout_log_id = $1
            )
            UPDATE exercise_log el
            SET position = ordered.new_position
            FROM ordered
            WHERE el.id = ordered.id
            "#,
        )
        .bind(session_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
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

#[async_trait]
impl WorkoutSessionQueries for WorkoutSessionRepositorySqlx {
    async fn find_current(&self, user_id: Uuid) -> Result<CurrentWorkoutSession, DomainError> {
        let row = sqlx::query_as::<_, CurrentWorkoutSessionRowModel>(
            r#"
            SELECT
                wl.id,
                wl.workout_plan_id,
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

        let exercises = sqlx::query_as::<_, WorkoutSessionDetailedExerciseRowModel>(
            r#"
            SELECT
                el.id,
                el.client_operation_id,
                e.id AS exercise_id,
                e.name AS exercise_name,
                e.exercise_type,
                e.equipment,
                e.muscle_group,
                el.position AS "order"
            FROM exercise_log el
            JOIN exercise e
                ON e.id = el.exercise_id
            WHERE el.workout_log_id = $1
                AND e.deleted_at IS NULL
            ORDER BY el.position ASC
            "#,
        )
        .bind(row.id)
        .fetch_all(&self.pool)
        .await?;

        let mut detailed_exercises = Vec::with_capacity(exercises.len());

        for exercise in exercises {
            let sets = sqlx::query_as::<_, WorkoutSessionSetRowModel>(
                r#"
                SELECT
                    id,
                    exercise_log_id AS session_exercise_id,
                    client_operation_id,
                    type AS set_type,
                    weight::real AS weight,
                    reps,
                    created_at
                FROM set_log
                WHERE exercise_log_id = $1
                ORDER BY created_at ASC
                "#,
            )
            .bind(exercise.id)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|row| WorkoutSessionSet {
                id: row.id,
                session_exercise_id: row.session_exercise_id,
                client_operation_id: row.client_operation_id,
                set_type: row.set_type.into(),
                weight: row.weight,
                reps: row.reps as u32,
                created_at: row.created_at,
            })
            .collect();

            detailed_exercises.push(WorkoutSessionDetailedExercise {
                id: exercise.id,
                client_operation_id: exercise.client_operation_id,
                exercise: WorkoutSessionExerciseDetails {
                    id: exercise.exercise_id,
                    name: exercise.exercise_name,
                    exercise_type: exercise.exercise_type.into(),
                    equipment: exercise.equipment.into(),
                    muscle_group: exercise.muscle_group.into(),
                },
                order: exercise.order,
                sets,
            });
        }

        Ok(CurrentWorkoutSession {
            id: row.id,
            workout_plan_id: row.workout_plan_id,
            workout_template_id: row.workout_template_id,
            workout_template_name: row.workout_template_name,
            started_at: row.started_at,
            status: row.status.into(),
            exercises: detailed_exercises,
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
                wl.status,
                agg.total_sets,
                agg.total_volume_kg
            FROM workout_log wl
            JOIN workout_template wt
                ON wt.id = wl.workout_template_id
            LEFT JOIN LATERAL (
                SELECT
                    COUNT(sl.id) AS total_sets,
                    COALESCE(SUM(sl.weight * sl.reps), 0)::float8 AS total_volume_kg
                FROM exercise_log el
                JOIN set_log sl
                    ON sl.exercise_log_id = el.id
                WHERE el.workout_log_id = wl.id
            ) agg ON TRUE
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
                total_sets: row.total_sets,
                total_volume_kg: row.total_volume_kg,
            })
            .collect())
    }

    async fn weekly_summary(
        &self,
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<WorkoutSessionWeeklySummary, DomainError> {
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

        let days = rows
            .into_iter()
            .map(|row| WorkoutSessionWeeklySummaryDay {
                date: row.date,
                day_of_week: row.day_of_week,
                trained: row.session_id.is_some(),
                session_id: row.session_id,
            })
            .collect();

        // Sessões canceladas recebem deleted_at, então já ficam de fora.
        let total_volume_kg: f64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(SUM(sl.weight * sl.reps), 0)::float8
            FROM set_log sl
            JOIN exercise_log el ON el.id = sl.exercise_log_id
            JOIN workout_log wl ON wl.id = el.workout_log_id
            WHERE wl.user_id = $1
                AND wl.deleted_at IS NULL
                AND wl.started_at::date BETWEEN $2 AND $3
            "#,
        )
        .bind(user_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&self.pool)
        .await?;

        Ok(WorkoutSessionWeeklySummary {
            days,
            total_volume_kg,
        })
    }
}
