use crate::adapters::http::dtos::FinishWorkoutSessionRequestDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_finished_response;
use crate::application::app_state::AppState;
use crate::application::dtos::workout_session::FinishWorkoutSessionInput;
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
        (status = 200, description = "Workout session finished", body = crate::adapters::http::dtos::FinishedWorkoutSessionResponseDTO),
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
        .execute(
            current_user,
            FinishWorkoutSessionInput {
                session_id,
                finished_at: request.finished_at,
            },
        )
        .await
    {
        Ok(session) => Json(to_finished_response(session)).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
