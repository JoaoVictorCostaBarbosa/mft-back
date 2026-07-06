use crate::application::errors::AppError;
use std::sync::Arc;

use crate::domain::entities::Measurement;
use crate::domain::entities::User;
use crate::domain::repositories::MeasurementRepository;

pub struct GetAllUserMeasurements {
    measurement_repo: Arc<dyn MeasurementRepository>,
}

impl GetAllUserMeasurements {
    pub fn new(measurement_repo: Arc<dyn MeasurementRepository>) -> Self {
        Self { measurement_repo }
    }

    pub async fn exexcute(&self, current_user: User) -> Result<Vec<Measurement>, AppError> {
        let result = self
            .measurement_repo
            .get_measurements_by_user_id(current_user.id)
            .await?;

        let measurements = result
            .into_iter()
            .filter(|m| m.deleted_at.is_none())
            .collect();

        Ok(measurements)
    }
}
