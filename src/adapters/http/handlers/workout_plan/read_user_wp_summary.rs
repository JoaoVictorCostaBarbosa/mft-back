use crate::{
    adapters::http::{
        dtos::workout_plan::WorkoutPlanSummaryResponseDTO, errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_plan_mapper::to_workout_plan_summary_response,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

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
