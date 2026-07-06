use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::ExerciseMapper;
use crate::application::app_state::AppState;
use axum::{Json, extract::State, response::IntoResponse};

#[utoipa::path{
    get,
    path = "/api/exercises/personal-records",
    responses(
        (status = 200, description = "Personal record (heaviest set) per exercise", body = crate::adapters::http::dtos::ExercisePersonalRecordsResponseDTO),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Exercises"
}]
pub async fn read_personal_records_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    match state
        .exercise
        .read_personal_records
        .execute(current_user)
        .await
    {
        Ok(records) => {
            Json(ExerciseMapper::personal_records_to_response(records)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
