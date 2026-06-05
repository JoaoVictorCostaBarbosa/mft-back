use crate::{
    adapters::http::{
        dtos::workout_session::AddSetToWorkoutSessionRequestDTO,
        errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_session_mapper::{to_set_response, to_set_type},
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
    post,
    path = "/api/workout-sessions/{session_id}/sets",
    request_body = AddSetToWorkoutSessionRequestDTO,
    params(("session_id" = Uuid, description = "Workout session ID")),
    responses(
        (status = 201, description = "Set added to workout session", body = crate::adapters::http::dtos::workout_session::WorkoutSessionSetResponseDTO),
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
            session_id,
            request.exercise_id,
            to_set_type(request.set_type),
            request.weight,
            request.reps,
        )
        .await
    {
        Ok(set) => (StatusCode::CREATED, Json(to_set_response(set))).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
