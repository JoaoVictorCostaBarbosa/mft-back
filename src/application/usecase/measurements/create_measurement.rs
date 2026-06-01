use crate::{
    application::dtos::measurements::create_measurement::CreateMeasurementRequest,
    domain::{
        entities::{measurement::Measurement, user::User},
        errors::domain_error::DomainError,
        repositories::measurement_repository::MeasurementRepository,
    },
};
use std::sync::Arc;

pub struct CreateMeasurement {
    pub measurement_repo: Arc<dyn MeasurementRepository>,
}

impl CreateMeasurement {
    pub fn new(measurement_repo: Arc<dyn MeasurementRepository>) -> Self {
        Self { measurement_repo }
    }

    pub async fn execute(
        &self,
        request: CreateMeasurementRequest,
        current_user: User,
    ) -> Result<Measurement, DomainError> {
        let measurement = Measurement::new(
            current_user.id,
            request.weight,
            request.height,
            request.left_calf,
            request.right_calf,
            request.left_quadriceps,
            request.right_quadriceps,
            request.hip,
            request.waist,
            request.chest,
            request.shoulders,
            request.left_arm,
            request.right_arm,
            request.left_forearm,
            request.right_forearm,
        )?;

        self.measurement_repo
            .create_measurement(measurement.clone())
            .await?;

        Ok(measurement)
    }
}
