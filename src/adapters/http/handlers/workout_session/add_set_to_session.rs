use crate::adapters::http::dtos::AddSetToWorkoutSessionRequestDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_set_response;
use crate::adapters::http::mappers::to_set_type;
use crate::application::app_state::AppState;
use crate::application::dtos::workout_session::AddSetToSessionInput;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    post,
    path = "/api/workout-sessions/{session_id}/sets",
    request_body = AddSetToWorkoutSessionRequestDTO,
    params(("session_id" = Uuid, description = "Workout session ID")),
    responses(
        (status = 201, description = "Set added to workout session", body = crate::adapters::http::dtos::WorkoutSessionSetResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 422, description = "invalid set"),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Workout Sessions"
}]
pub async fn add_set_to_workout_session_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(session_id): Path<Uuid>,
    Json(request): Json<AddSetToWorkoutSessionRequestDTO>,
) -> impl IntoResponse {
    match state
        .workout_session
        .add_set
        .execute(
            current_user,
            AddSetToSessionInput {
                session_id,
                exercise_id: request.exercise_id,
                set_type: to_set_type(request.set_type),
                weight: request.weight,
                reps: request.reps,
                client_operation_id: request.client_operation_id,
                completed_at: request.completed_at,
            },
        )
        .await
    {
        Ok(set) => (StatusCode::CREATED, Json(to_set_response(set))).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
