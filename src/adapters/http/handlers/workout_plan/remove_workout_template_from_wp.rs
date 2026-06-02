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

#[utoipa::path{
    delete,
    path = "/api/workout-plans/{workout_plan_id}/workout-template/{workout_template_id}",
    params(
        ("workout_plan_id" = Uuid, description = "Workout plan ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
        ("workout_template_id" = Uuid, description = "Workout template ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
    ),
    responses(
        (status = 204, description = "Workout template removed from workout plan"),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Plans"
}]
pub async fn remove_workout_template_from_workout_plan_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path((workout_plan_id, workout_template_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match state
        .workout_plan
        .remove_workout_template
        .execute(current_user, workout_plan_id, workout_template_id)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
