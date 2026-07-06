use crate::adapters::http::dtos::AddExerciseToWorkoutSessionRequestDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_exercise_response;
use crate::application::app_state::AppState;
use crate::application::dtos::workout_session::AddExerciseToSessionInput;
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
        (status = 201, description = "Exercise added to workout session", body = crate::adapters::http::dtos::WorkoutSessionExerciseResponseDTO),
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
        .execute(
            current_user,
            AddExerciseToSessionInput {
                session_id,
                exercise_id: request.exercise_id,
                client_operation_id: request.client_operation_id,
            },
        )
        .await
    {
        Ok(exercise) => (StatusCode::CREATED, Json(to_exercise_response(exercise))).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
