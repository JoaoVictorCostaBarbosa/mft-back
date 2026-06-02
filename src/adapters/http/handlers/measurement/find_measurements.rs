use crate::{
    adapters::http::{
        dtos::measurement_dto::MeasurementResponse, errors::http_error::HttpError,
        extractors::current_user::CurrentUser, mappers::measurement_mapper::MeasurementMapper,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    get,
    path = "/api/measurements",
    responses(
        (status = 200, description = "Measurements found", body = [MeasurementResponse]),
        (status = 403, description = "denied permission"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Measurements"
}]
pub async fn find_measurements_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    match state.measurement.get_all.exexcute(current_user).await {
        Ok(measurements) => {
            let response: Vec<MeasurementResponse> = measurements
                .into_iter()
                .map(|m| MeasurementMapper::domain_to_response(m))
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
