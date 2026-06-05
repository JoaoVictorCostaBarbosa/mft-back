use crate::{
    adapters::http::{
        errors::http_error::HttpError, extractors::current_user::CurrentUser,
        mappers::workout_session_mapper::to_current_response,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, response::IntoResponse};

#[utoipa::path{
    get,
    path = "/api/workout-sessions/current",
    responses(
        (status = 200, description = "Current workout session found", body = crate::adapters::http::dtos::workout_session::CurrentWorkoutSessionResponseDTO),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Workout Sessions"
}]
pub async fn find_current_workout_session_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    match state
        .workout_session
        .find_current
        .execute(current_user)
        .await
    {
        Ok(session) => Json(to_current_response(session)).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
