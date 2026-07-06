use crate::adapters::http::dtos::CreateMeasurementDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::MeasurementMapper;
use crate::application::app_state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    post,
    path = "/api/measurements",
    request_body = CreateMeasurementDTO,
    responses(
        (status = 201, description = "Measurement created", body = MeasurementResponse),
        (status = 403, description = "denied permission"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Measurements"
}]
pub async fn create_measurement_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<CreateMeasurementDTO>,
) -> impl IntoResponse {
    match state
        .measurement
        .create
        .execute(MeasurementMapper::dto_to_request(request), current_user)
        .await
    {
        Ok(measurement) => (
            StatusCode::CREATED,
            Json(MeasurementMapper::domain_to_response(measurement)),
        )
            .into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
