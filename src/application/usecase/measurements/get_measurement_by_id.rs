use crate::application::errors::AppError;
use crate::domain::entities::Measurement;
use crate::domain::entities::User;
use crate::domain::errors::PermissionError;
use crate::domain::errors::RepositoryError;
use crate::domain::repositories::MeasurementRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetMeasurementById {
    measurement_repo: Arc<dyn MeasurementRepository>,
}

impl GetMeasurementById {
    pub fn new(measurement_repo: Arc<dyn MeasurementRepository>) -> Self {
        Self { measurement_repo }
    }

    pub async fn execute(&self, id: Uuid, current_user: User) -> Result<Measurement, AppError> {
        let measurement = self.measurement_repo.get_measurement_by_id(id).await?;

        if measurement.user_id != current_user.id {
            return Err(PermissionError::Forbidden)?;
        }

        if measurement.deleted_at.is_some() {
            return Err(RepositoryError::NotFound(
                "measurement not found".to_string(),
            ))?;
        }

        Ok(measurement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Measurement;
    use crate::domain::errors::DomainError;
    use crate::test_support::fakes::InMemoryMeasurementRepository;
    use crate::test_support::fixtures;
    use std::sync::Arc;

    fn measurement(user_id: Uuid) -> Measurement {
        Measurement::new(
            user_id,
            Some(80.0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap()
    }

    #[tokio::test]
    async fn owner_reads_own_measurement() {
        let user = fixtures::user();
        let m = measurement(user.id);
        let id = m.id;
        let repo = Arc::new(InMemoryMeasurementRepository::with_measurements(vec![m]));
        let use_case = GetMeasurementById::new(repo);

        let found = use_case.execute(id, user).await.unwrap();

        assert_eq!(found.id, id);
    }

    #[tokio::test]
    async fn rejects_measurement_of_another_user() {
        let m = measurement(Uuid::new_v4());
        let id = m.id;
        let repo = Arc::new(InMemoryMeasurementRepository::with_measurements(vec![m]));
        let use_case = GetMeasurementById::new(repo);

        let err = use_case.execute(id, fixtures::user()).await.unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
    }

    #[tokio::test]
    async fn soft_deleted_measurement_is_not_found() {
        let user = fixtures::user();
        let mut m = measurement(user.id);
        m.deleted_at = Some(chrono::Utc::now());
        let id = m.id;
        let repo = Arc::new(InMemoryMeasurementRepository::with_measurements(vec![m]));
        let use_case = GetMeasurementById::new(repo);

        let err = use_case.execute(id, user).await.unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Repository(RepositoryError::NotFound(_)))
        ));
    }
}
