use crate::{
    adapters::http::{
        errors::http_error::HttpError, extractors::current_user::CurrentUser,
        mappers::workout_plan_mapper::to_workout_plan_response,
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

pub async fn find_workout_plan_by_id_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(workout_plan_id): Path<Uuid>,
) -> impl IntoResponse {
    match state
        .workout_plan
        .find_by_id
        .execute(current_user, workout_plan_id)
        .await
    {
        Ok(wp) => {
            let response = to_workout_plan_response(wp);

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
