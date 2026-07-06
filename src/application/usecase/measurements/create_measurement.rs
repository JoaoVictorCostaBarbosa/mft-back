use crate::application::dtos::measurements::CreateMeasurementRequest;
use crate::application::errors::AppError;
use crate::domain::entities::Measurement;
use crate::domain::entities::User;
use crate::domain::repositories::MeasurementRepository;
use std::sync::Arc;

pub struct CreateMeasurement {
    measurement_repo: Arc<dyn MeasurementRepository>,
}

impl CreateMeasurement {
    pub fn new(measurement_repo: Arc<dyn MeasurementRepository>) -> Self {
        Self { measurement_repo }
    }

    pub async fn execute(
        &self,
        request: CreateMeasurementRequest,
        current_user: User,
    ) -> Result<Measurement, AppError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::DomainError;
    use crate::test_support::fakes::InMemoryMeasurementRepository;
    use crate::test_support::fixtures;
    use std::sync::Arc;

    fn request(weight: Option<f32>) -> CreateMeasurementRequest {
        CreateMeasurementRequest {
            weight,
            height: None,
            left_calf: None,
            right_calf: None,
            left_quadriceps: None,
            right_quadriceps: None,
            hip: None,
            waist: None,
            chest: None,
            shoulders: None,
            left_arm: None,
            right_arm: None,
            left_forearm: None,
            right_forearm: None,
        }
    }

    #[tokio::test]
    async fn creates_measurement_for_current_user() {
        let repo = Arc::new(InMemoryMeasurementRepository::default());
        let use_case = CreateMeasurement::new(repo.clone());
        let user = fixtures::user();
        let user_id = user.id;

        let measurement = use_case.execute(request(Some(80.0)), user).await.unwrap();

        assert_eq!(measurement.user_id, user_id);
        assert_eq!(repo.measurements.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn rejects_invalid_weight() {
        let repo = Arc::new(InMemoryMeasurementRepository::default());
        let use_case = CreateMeasurement::new(repo.clone());

        let err = use_case
            .execute(request(Some(-10.0)), fixtures::user())
            .await
            .unwrap_err();

        assert!(matches!(err, AppError::Domain(DomainError::Measurement(_))));
        assert!(repo.measurements.lock().unwrap().is_empty());
    }
}
