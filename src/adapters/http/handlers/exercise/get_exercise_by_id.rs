use crate::{
    adapters::http::{
        errors::http_error::HttpError, extractors::current_user::CurrentUser,
        mappers::exercise_mapper::ExerciseMapper,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn get_exercise_by_id_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.exercise.get_by_id.execute(id, current_user).await {
        Ok(exercise) => (
            StatusCode::OK,
            Json(ExerciseMapper::domain_to_response(exercise)),
        )
            .into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
