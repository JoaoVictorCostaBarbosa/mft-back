use crate::{
    adapters::http::{errors::http_error::HttpError, extractors::current_user::CurrentUser},
    application::app_state::app_state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn delete_exercise_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.exercise.delete.execute(id, current_user).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
