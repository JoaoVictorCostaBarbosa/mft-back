use crate::application::usecase::measurements::CreateMeasurement;
use crate::application::usecase::measurements::DeleteMeasurement;
use crate::application::usecase::measurements::GetAllUserMeasurements;
use crate::application::usecase::measurements::GetMeasurementById;
use crate::application::usecase::measurements::SoftDeleteMeasurement;
use crate::domain::repositories::MeasurementRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct MeasurementAppState {
    pub create: Arc<CreateMeasurement>,
    pub get_all: Arc<GetAllUserMeasurements>,
    pub get_by_id: Arc<GetMeasurementById>,
    pub soft_delete: Arc<SoftDeleteMeasurement>,
    pub delete: Arc<DeleteMeasurement>,
}

impl MeasurementAppState {
    pub fn new(measurement_repo: Arc<dyn MeasurementRepository>) -> Self {
        Self {
            create: Arc::new(CreateMeasurement::new(measurement_repo.clone())),
            get_all: Arc::new(GetAllUserMeasurements::new(measurement_repo.clone())),
            get_by_id: Arc::new(GetMeasurementById::new(measurement_repo.clone())),
            soft_delete: Arc::new(SoftDeleteMeasurement::new(measurement_repo.clone())),
            delete: Arc::new(DeleteMeasurement::new(measurement_repo.clone())),
        }
    }
}
