use crate::adapters::http::dtos::WorkoutSessionWeeklySummaryQueryDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_weekly_summary_response;
use crate::application::app_state::AppState;
use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

#[utoipa::path{
    get,
    path = "/api/workout-sessions/weekly-summary",
    params(WorkoutSessionWeeklySummaryQueryDTO),
    responses(
        (status = 200, description = "Workout session weekly summary", body = crate::adapters::http::dtos::WorkoutSessionWeeklySummaryResponseDTO),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Workout Sessions"
}]
pub async fn read_workout_session_weekly_summary_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Query(query): Query<WorkoutSessionWeeklySummaryQueryDTO>,
) -> impl IntoResponse {
    match state
        .workout_session
        .weekly_summary
        .execute(current_user, query.start_date, query.end_date)
        .await
    {
        Ok(days) => Json(to_weekly_summary_response(days)).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
