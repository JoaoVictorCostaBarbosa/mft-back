use crate::domain::entities::Measurement;
use crate::domain::errors::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait MeasurementRepository: Send + Sync {
    async fn create_measurement(&self, measurement: Measurement) -> Result<(), DomainError>;
    async fn get_measurement_by_id(&self, id: Uuid) -> Result<Measurement, DomainError>;
    async fn get_measurements_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Measurement>, DomainError>;
    async fn soft_delete_measurement(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_measurement(&self, id: Uuid) -> Result<(), DomainError>;
}
