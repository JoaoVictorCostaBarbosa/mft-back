use crate::adapters::http::dtos::CreateMeasurementDTO;
use crate::adapters::http::dtos::MeasurementResponse;
use crate::application::dtos::measurements::CreateMeasurementRequest;
use crate::domain::entities::Measurement;

pub struct MeasurementMapper;

impl MeasurementMapper {
    pub fn dto_to_request(request: CreateMeasurementDTO) -> CreateMeasurementRequest {
        CreateMeasurementRequest {
            weight: request.weight,
            height: request.height,
            left_calf: request.left_calf,
            right_calf: request.right_calf,
            left_quadriceps: request.left_quadriceps,
            right_quadriceps: request.right_quadriceps,
            hip: request.hip,
            waist: request.waist,
            chest: request.chest,
            shoulders: request.shoulders,
            left_arm: request.left_arm,
            right_arm: request.right_arm,
            left_forearm: request.left_forearm,
            right_forearm: request.right_forearm,
        }
    }

    pub fn domain_to_response(request: Measurement) -> MeasurementResponse {
        MeasurementResponse {
            id: request.id,
            created_at: request.created_at,
            weight: request.weight.map(|v| v.value()),
            height: request.height.map(|v| v.value()),
            left_calf: request.left_calf.map(|v| v.value()),
            right_calf: request.right_calf.map(|v| v.value()),
            left_quadriceps: request.left_quadriceps.map(|v| v.value()),
            right_quadriceps: request.right_quadriceps.map(|v| v.value()),
            hip: request.hip.map(|v| v.value()),
            waist: request.waist.map(|v| v.value()),
            chest: request.chest.map(|v| v.value()),
            shoulders: request.shoulders.map(|v| v.value()),
            left_arm: request.left_arm.map(|v| v.value()),
            right_arm: request.right_arm.map(|v| v.value()),
            left_forearm: request.left_forearm.map(|v| v.value()),
            right_forearm: request.right_forearm.map(|v| v.value()),
        }
    }
}
