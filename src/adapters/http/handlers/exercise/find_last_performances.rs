use crate::adapters::http::dtos::ExerciseLastPerformancesRequestDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::ExerciseMapper;
use crate::application::app_state::AppState;
use axum::{Json, extract::State, response::IntoResponse};

#[utoipa::path{
    post,
    path = "/api/exercises/last-performances",
    request_body = ExerciseLastPerformancesRequestDTO,
    responses(
        (status = 200, description = "Last finished workout performance by exercise", body = crate::adapters::http::dtos::ExerciseLastPerformancesResponseDTO),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Exercises"
}]
pub async fn find_exercise_last_performances_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<ExerciseLastPerformancesRequestDTO>,
) -> impl IntoResponse {
    match state
        .exercise
        .find_last_performances
        .execute(current_user, request.exercise_ids)
        .await
    {
        Ok(performances) => {
            Json(ExerciseMapper::last_performances_to_response(performances)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
