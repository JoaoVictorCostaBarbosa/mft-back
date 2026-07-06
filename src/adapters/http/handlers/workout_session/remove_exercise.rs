use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::application::app_state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    delete,
    path = "/api/workout-sessions/{session_id}/exercises/{session_exercise_id}",
    params(
        ("session_id" = Uuid, description = "Workout session ID"),
        ("session_exercise_id" = Uuid, description = "Session exercise ID")
    ),
    responses(
        (status = 204, description = "Exercise removed from workout session"),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 409, description = "session is not editable"),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Workout Sessions"
}]
pub async fn remove_exercise_from_workout_session_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path((session_id, session_exercise_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match state
        .workout_session
        .remove_exercise
        .execute(current_user, session_id, session_exercise_id)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
