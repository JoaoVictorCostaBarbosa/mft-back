use crate::adapters::http::dtos::ExerciseRequest;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::ExerciseMapper;
use crate::application::app_state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    post,
    path = "/api/exercises",
    request_body = ExerciseRequest,
    responses(
        (status = 201, description = "Exercise created", body = ExerciseResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Exercises"
}]
pub async fn create_exercise_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<ExerciseRequest>,
) -> impl IntoResponse {
    match state
        .exercise
        .create
        .execute(ExerciseMapper::dto_to_request(request), current_user)
        .await
    {
        Ok(exercise) => (
            StatusCode::CREATED,
            Json(ExerciseMapper::domain_to_response(exercise)),
        )
            .into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
