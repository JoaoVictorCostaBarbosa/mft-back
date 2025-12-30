use crate::{
    domain::{
        commands::exercise_commands::{ExerciseFilterFields, ExerciseUpdateFields},
        entities::exercise::Exercise,
        errors::{domain_error::DomainError, repository_error::RepositoryError},
        repositories::exercise_repository::ExerciseRepository,
    },
    infrastructure::repositories::{
        enums_db::{
            equipment_db::EquipmentDb, exercise_type_db::ExerciseTypeDb,
            muscle_group_db::MuscleGroupDb,
        },
        models::exercise_model::ExerciseModel,
    },
};
use axum::async_trait;
use chrono::Utc;
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

pub struct ExerciseRepositorySqlx {
    pub pool: PgPool,
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
    ) -> Result<Vec<Exercise>, DomainError> {
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

        let models: Vec<ExerciseModel> = qb.build_query_as().fetch_all(&self.pool).await?;

        let exercises = models
            .into_iter()
            .map(Exercise::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(exercises)
    }

    async fn update_exercise(
        &self,
        fields: ExerciseUpdateFields,
        user_id: Option<Uuid>,
    ) -> Result<(), DomainError> {
        let mut qb = QueryBuilder::new("UPDATE exercise SET ");
        let mut separated = false;
    
        if let Some(name) = fields.name {
            if separated { qb.push(", "); }
            qb.push("name = ");
            qb.push_bind(name);
            separated = true;
        }
    
        if let Some(exercise_type) = fields.exercise_type {
            if separated { qb.push(", "); }
            qb.push("exercise_type = ");
            qb.push_bind(ExerciseTypeDb::from(exercise_type));
            separated = true;
        }
    
        if let Some(equipment) = fields.equipment {
            if separated { qb.push(", "); }
            qb.push("equipment = ");
            qb.push_bind(EquipmentDb::from(equipment));
            separated = true;
        }
    
        if let Some(muscle_group) = fields.muscle_group {
            if separated { qb.push(", "); }
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
}
