use crate::{
    adapters::http::{errors::http_error::HttpError, extractors::current_user::CurrentUser},
    application::app_state::app_state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    patch,
    path = "/api/exercises/{id}/soft-delete",
    params(
        ("id" = Uuid, description = "Exercise ID to be soft deleted", example = "b728b759-4d32-4148-936e-d9036c071d72"),
    ),
    responses(
        (status = 204, description = "no content"),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Exercises"
}]
pub async fn soft_delete_exercise_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.exercise.soft_delete.execute(id, current_user).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
