use crate::{
    adapters::http::{
        dtos::workout_template::WorkoutTemplateSummaryResponse, errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_template_mapper::to_response_workout_templalte_summary,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn read_user_workout_templates_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    match state.workout_template.read.execute(current_user).await {
        Ok(workouts_template) => {
            let response: Vec<WorkoutTemplateSummaryResponse> = workouts_template
                .into_iter()
                .map(|wt| to_response_workout_templalte_summary(wt))
                .collect();

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
