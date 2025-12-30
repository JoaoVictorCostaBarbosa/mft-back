use crate::{
    adapters::http::{
        dtos::exercise_dto::ExerciseResponse, errors::http_error::HttpError,
        extractors::current_user::CurrentUser, mappers::exercise_mapper::ExerciseMapper,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn read_exercises_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    match state.exercise.read.execute(current_user).await {
        Ok(exercises) => {
            let response: Vec<ExerciseResponse> = exercises
                .into_iter()
                .map(|e| ExerciseMapper::domain_to_response(e))
                .collect();

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
