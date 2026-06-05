use crate::{
    adapters::http::{
        dtos::workout_session::ReorderWorkoutSessionExercisesRequestDTO,
        errors::http_error::HttpError, extractors::current_user::CurrentUser,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    patch,
    path = "/api/workout-sessions/{session_id}/exercises/reorder",
    request_body = ReorderWorkoutSessionExercisesRequestDTO,
    params(("session_id" = Uuid, description = "Workout session ID")),
    responses(
        (status = 204, description = "Workout session exercises reordered"),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 409, description = "session is not editable"),
        (status = 422, description = "invalid exercise order"),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Workout Sessions"
}]
pub async fn reorder_workout_session_exercises_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(session_id): Path<Uuid>,
    Json(request): Json<ReorderWorkoutSessionExercisesRequestDTO>,
) -> impl IntoResponse {
    match state
        .workout_session
        .reorder_exercises
        .execute(
            current_user,
            session_id,
            request.ordered_session_exercise_ids,
        )
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
