use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::ExerciseMapper;
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
    path = "/api/exercises/{id}",
    params(
        ("id" = Uuid, description = "Exercise ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
    ),
    responses(
        (status = 200, description = "Exercise found", body = ExerciseResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Exercises"
}]
pub async fn get_exercise_by_id_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.exercise.get_by_id.execute(id, current_user).await {
        Ok(exercise) => (
            StatusCode::OK,
            Json(ExerciseMapper::domain_to_response(exercise)),
        )
            .into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
