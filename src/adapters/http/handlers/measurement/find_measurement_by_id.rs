use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::MeasurementMapper;
use crate::application::app_state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    get,
    path = "/api/measurements/{id}",
    params(
        ("id" = Uuid, description = "Measurement ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
    ),
    responses(
        (status = 200, description = "Measurement found", body = MeasurementResponse),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Measurements"
}]
pub async fn find_measurement_by_id_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.measurement.get_by_id.execute(id, current_user).await {
        Ok(measurement) => (
            StatusCode::OK,
            Json(MeasurementMapper::domain_to_response(measurement)),
        )
            .into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
