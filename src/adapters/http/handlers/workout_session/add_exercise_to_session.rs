use crate::{
    adapters::http::{
        dtos::workout_session::AddExerciseToWorkoutSessionRequestDTO,
        errors::http_error::HttpError, extractors::current_user::CurrentUser,
        mappers::workout_session_mapper::to_exercise_response,
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
    path = "/api/workout-sessions/{session_id}/exercises",
    request_body = AddExerciseToWorkoutSessionRequestDTO,
    params(("session_id" = Uuid, description = "Workout session ID")),
    responses(
        (status = 201, description = "Exercise added to workout session", body = crate::adapters::http::dtos::workout_session::WorkoutSessionExerciseResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Workout Sessions"
}]
pub async fn add_exercise_to_workout_session_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(session_id): Path<Uuid>,
    Json(request): Json<AddExerciseToWorkoutSessionRequestDTO>,
) -> impl IntoResponse {
    match state
        .workout_session
        .add_exercise
        .execute(current_user, session_id, request.exercise_id)
        .await
    {
        Ok(exercise) => (StatusCode::CREATED, Json(to_exercise_response(exercise))).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
