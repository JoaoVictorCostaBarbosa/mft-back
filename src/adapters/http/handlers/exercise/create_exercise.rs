use crate::{
    adapters::http::{
        dtos::exercise_dto::ExerciseRequest, errors::http_error::HttpError,
        extractors::current_user::CurrentUser, mappers::exercise_mapper::ExerciseMapper,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn create_exercise_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<ExerciseRequest>,
) -> impl IntoResponse {
    match state
        .exercise
        .create
        .execute(ExerciseMapper::dto_to_request(request), current_user)
        .await
    {
        Ok(exercise) => (
            StatusCode::CREATED,
            Json(ExerciseMapper::domain_to_response(exercise)),
        )
            .into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
