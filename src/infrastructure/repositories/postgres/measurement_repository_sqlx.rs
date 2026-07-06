use crate::domain::entities::Measurement;
use crate::domain::errors::DomainError;
use crate::domain::errors::MeasurementError;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::MeasurementRepository;
use crate::infrastructure::repositories::models::MeasurementModel;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct MeasurementRepositorySqlx {
    pool: PgPool,
}

impl MeasurementRepositorySqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MeasurementRepository for MeasurementRepositorySqlx {
    async fn create_measurement(&self, measurement: Measurement) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO measurements
            (
                id,
                user_id,
                weight,
                height,
                left_calf,
                right_calf,
                left_quadriceps,
                right_quadriceps,
                hip,
                waist,
                chest,
                shoulders,
                left_arm,
                right_arm,
                left_forearm,
                right_forearm,
                created_at,
                deleted_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9,
                $10, $11, $12, $13, $14, $15, $16,
                $17, $18
            )
            "#,
        )
        .bind(measurement.id)
        .bind(measurement.user_id)
        .bind(measurement.weight.map(|v| v.value()))
        .bind(measurement.height.map(|v| v.value()))
        .bind(measurement.left_calf.map(|v| v.value()))
        .bind(measurement.right_calf.map(|v| v.value()))
        .bind(measurement.left_quadriceps.map(|v| v.value()))
        .bind(measurement.right_quadriceps.map(|v| v.value()))
        .bind(measurement.hip.map(|v| v.value()))
        .bind(measurement.waist.map(|v| v.value()))
        .bind(measurement.chest.map(|v| v.value()))
        .bind(measurement.shoulders.map(|v| v.value()))
        .bind(measurement.left_arm.map(|v| v.value()))
        .bind(measurement.right_arm.map(|v| v.value()))
        .bind(measurement.left_forearm.map(|v| v.value()))
        .bind(measurement.right_forearm.map(|v| v.value()))
        .bind(measurement.created_at)
        .bind(measurement.deleted_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_measurement_by_id(&self, id: Uuid) -> Result<Measurement, DomainError> {
        let result = sqlx::query_as::<_, MeasurementModel>(
            r#"
            SELECT
                id,
                user_id,
                weight::float8 AS weight,
                height::float8 AS height,
                left_calf::float8 AS left_calf,
                right_calf::float8 AS right_calf,
                left_quadriceps::float8 AS left_quadriceps,
                right_quadriceps::float8 AS right_quadriceps,
                hip::float8 AS hip,
                waist::float8 AS waist,
                chest::float8 AS chest,
                shoulders::float8 AS shoulders,
                left_arm::float8 AS left_arm,
                right_arm::float8 AS right_arm,
                left_forearm::float8 AS left_forearm,
                right_forearm::float8 AS right_forearm,
                created_at,
                deleted_at
            FROM measurements
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        let measurement = result.to_domain()?;

        Ok(measurement)
    }

    async fn get_measurements_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Measurement>, DomainError> {
        let result = sqlx::query_as::<_, MeasurementModel>(
            r#"
            SELECT
                id,
                user_id,
                weight::float8 AS weight,
                height::float8 AS height,
                left_calf::float8 AS left_calf,
                right_calf::float8 AS right_calf,
                left_quadriceps::float8 AS left_quadriceps,
                right_quadriceps::float8 AS right_quadriceps,
                hip::float8 AS hip,
                waist::float8 AS waist,
                chest::float8 AS chest,
                shoulders::float8 AS shoulders,
                left_arm::float8 AS left_arm,
                right_arm::float8 AS right_arm,
                left_forearm::float8 AS left_forearm,
                right_forearm::float8 AS right_forearm,
                created_at,
                deleted_at
            FROM measurements
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let measurements = result
            .into_iter()
            .map(|m| m.to_domain())
            .collect::<Result<Vec<_>, MeasurementError>>()?;

        Ok(measurements)
    }

    async fn soft_delete_measurement(&self, id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            UPDATE measurements
            SET deleted_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound("measurement not found".to_string()).into());
        }

        Ok(())
    }

    async fn delete_measurement(&self, id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query(
            r#"
            DELETE FROM measurements
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound("measurement not found".to_string()).into());
        }

        Ok(())
    }
}
