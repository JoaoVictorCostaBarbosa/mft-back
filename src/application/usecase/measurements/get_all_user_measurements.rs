use std::sync::Arc;

use crate::domain::{
    entities::{measurement::Measurement, user::User},
    errors::domain_error::DomainError,
    repositories::measurement_repository::MeasurementRepository,
};

pub struct GetAllUserMeasurements {
    pub measurement_repo: Arc<dyn MeasurementRepository>,
}

impl GetAllUserMeasurements {
    pub fn new(measurement_repo: Arc<dyn MeasurementRepository>) -> Self {
        Self { measurement_repo }
    }

    pub async fn exexcute(&self, current_user: User) -> Result<Vec<Measurement>, DomainError> {
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
