use crate::{
    adapters::http::{
        dtos::workout_template::WorkoutTemplateExerciseDTO, errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_template_mapper::to_request_workout_template_exercise,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn add_exercise_to_workout_template_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<WorkoutTemplateExerciseDTO>,
) -> impl IntoResponse {
    let wte = to_request_workout_template_exercise(request);

    match state
        .workout_template
        .add_exercise
        .execute(current_user, wte)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
