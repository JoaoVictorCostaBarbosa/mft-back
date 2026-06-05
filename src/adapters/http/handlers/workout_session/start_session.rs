use crate::{
    adapters::http::{
        dtos::workout_session::StartWorkoutSessionRequestDTO, errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_session_mapper::to_session_response,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    post,
    path = "/api/workout-sessions",
    request_body = StartWorkoutSessionRequestDTO,
    responses(
        (status = 201, description = "Workout session started", body = crate::adapters::http::dtos::workout_session::WorkoutSessionResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 409, description = "session already in progress"),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Workout Sessions"
}]
pub async fn start_workout_session_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<StartWorkoutSessionRequestDTO>,
) -> impl IntoResponse {
    match state
        .workout_session
        .start
        .execute(
            current_user,
            request.workout_plan_id,
            request.workout_template_id,
        )
        .await
    {
        Ok(session) => (StatusCode::CREATED, Json(to_session_response(session))).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
