use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_workout_plan_response;
use crate::application::app_state::AppState;

#[utoipa::path{
    get,
    path = "/api/workout-plans/current",
    responses(
        (status = 200, description = "Current workout plan found", body = WorkoutPlanResponseDTO),
        (status = 401, description = "unauthorized"),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Plans"
}]
pub async fn find_user_current_workout_plan(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    match state.workout_plan.find_current.execute(current_user).await {
        Ok(wp) => {
            let response = to_workout_plan_response(wp);

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
