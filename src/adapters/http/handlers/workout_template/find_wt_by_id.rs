use crate::{
    adapters::http::{
        errors::http_error::HttpError, extractors::current_user::CurrentUser,
        mappers::workout_template_mapper::to_response_workout_template,
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

pub async fn find_workout_template_by_id_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(workout_id): Path<Uuid>,
) -> impl IntoResponse {
    match state
        .workout_template
        .find_by_id
        .execute(current_user, workout_id)
        .await
    {
        Ok(wt) => {
            let response = to_response_workout_template(wt);

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
