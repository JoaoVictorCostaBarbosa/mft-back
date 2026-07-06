use crate::adapters::http::dtos::WorkoutPlanSummaryResponseDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_workout_plan_summary_response;
use crate::application::app_state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    get,
    path = "/api/workout-plans",
    responses(
        (status = 200, description = "Workout plans found", body = [WorkoutPlanSummaryResponseDTO]),
        (status = 403, description = "denied permission"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Plans"
}]
pub async fn read_user_workout_plans_summary_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    match state.workout_plan.read.execute(current_user).await {
        Ok(wps) => {
            let response: Vec<WorkoutPlanSummaryResponseDTO> = wps
                .into_iter()
                .map(|wp| to_workout_plan_summary_response(wp))
                .collect();

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
