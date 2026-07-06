use crate::application::errors::AppError;
use crate::domain::entities::User;
use crate::domain::enums::Role;
use crate::domain::errors::PermissionError;
use crate::domain::repositories::MeasurementRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteMeasurement {
    measurement_repo: Arc<dyn MeasurementRepository>,
}

impl DeleteMeasurement {
    pub fn new(measurement_repo: Arc<dyn MeasurementRepository>) -> Self {
        Self { measurement_repo }
    }

    pub async fn execute(&self, id: Uuid, current_user: User) -> Result<(), AppError> {
        if current_user.role != Role::Admin {
            return Err(PermissionError::Forbidden)?;
        }

        self.measurement_repo.delete_measurement(id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::DomainError;
    use crate::test_support::fakes::InMemoryMeasurementRepository;
    use crate::test_support::fixtures;
    use std::sync::Arc;

    #[tokio::test]
    async fn non_admin_cannot_delete() {
        let repo = Arc::new(InMemoryMeasurementRepository::default());
        let use_case = DeleteMeasurement::new(repo.clone());

        let err = use_case
            .execute(Uuid::new_v4(), fixtures::user())
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            AppError::Domain(DomainError::Permission(PermissionError::Forbidden))
        ));
        assert!(repo.deleted.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn admin_deletes_measurement() {
        let repo = Arc::new(InMemoryMeasurementRepository::default());
        let use_case = DeleteMeasurement::new(repo.clone());
        let id = Uuid::new_v4();

        use_case.execute(id, fixtures::admin()).await.unwrap();

        assert_eq!(repo.deleted.lock().unwrap().as_slice(), &[id]);
    }
}
