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

pub async fn delete_workout_plan_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(workout_plan_id): Path<Uuid>,
) -> impl IntoResponse {
    match state
        .workout_plan
        .delete
        .execute(current_user, workout_plan_id)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
