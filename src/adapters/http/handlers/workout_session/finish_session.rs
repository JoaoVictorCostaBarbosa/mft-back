use crate::{
    adapters::http::{
        dtos::workout_session::FinishWorkoutSessionRequestDTO, errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_session_mapper::to_finished_response,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    patch,
    path = "/api/workout-sessions/{session_id}/finish",
    request_body = FinishWorkoutSessionRequestDTO,
    params(("session_id" = Uuid, description = "Workout session ID")),
    responses(
        (status = 200, description = "Workout session finished", body = crate::adapters::http::dtos::workout_session::FinishedWorkoutSessionResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 422, description = "invalid finished_at"),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Workout Sessions"
}]
pub async fn finish_workout_session_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(session_id): Path<Uuid>,
    Json(request): Json<FinishWorkoutSessionRequestDTO>,
) -> impl IntoResponse {
    match state
        .workout_session
        .finish
        .execute(current_user, session_id, request.finished_at)
        .await
    {
        Ok(session) => Json(to_finished_response(session)).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
