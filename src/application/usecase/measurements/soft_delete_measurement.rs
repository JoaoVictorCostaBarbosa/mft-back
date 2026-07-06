use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::errors::PermissionError;
use crate::domain::repositories::MeasurementRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct SoftDeleteMeasurement {
    measurement_repo: Arc<dyn MeasurementRepository>,
}

impl SoftDeleteMeasurement {
    pub fn new(measurement_repo: Arc<dyn MeasurementRepository>) -> Self {
        Self { measurement_repo }
    }

    pub async fn execute(&self, id: Uuid, current_user: User) -> Result<(), AppError> {
        let measurement = self.measurement_repo.get_measurement_by_id(id).await?;

        if measurement.user_id != current_user.id {
            return Err(PermissionError::Forbidden)?;
        }

        self.measurement_repo.soft_delete_measurement(id).await?;

        Ok(())
    }
}
