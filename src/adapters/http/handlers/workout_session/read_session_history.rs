use crate::{
    adapters::http::{
        errors::http_error::HttpError, extractors::current_user::CurrentUser,
        mappers::workout_session_mapper::to_history_response,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, response::IntoResponse};

#[utoipa::path{
    get,
    path = "/api/workout-sessions/history",
    responses(
        (status = 200, description = "Workout session history", body = crate::adapters::http::dtos::workout_session::WorkoutSessionHistoryResponseDTO),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Workout Sessions"
}]
pub async fn read_workout_session_history_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    match state.workout_session.history.execute(current_user).await {
        Ok(history) => Json(to_history_response(history)).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
