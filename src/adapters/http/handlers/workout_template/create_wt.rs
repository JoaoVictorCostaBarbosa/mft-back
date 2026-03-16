use crate::{
    adapters::http::{
        dtos::workout_template::WorkoutTemplateRequestDTO,
        errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_template_mapper::{
            to_request_workout_template, to_response_workout_template,
        },
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn create_workout_template_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<WorkoutTemplateRequestDTO>,
) -> impl IntoResponse {
    let wt = to_request_workout_template(request);

    match state
        .workout_template
        .create
        .execute(current_user, wt)
        .await
    {
        Ok(template) => {
            let response = to_response_workout_template(template);

            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
