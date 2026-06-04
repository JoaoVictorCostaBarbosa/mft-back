use crate::{
    adapters::http::{
        errors::http_error::HttpError, extractors::current_user::CurrentUser,
        mappers::workout_plan_mapper::to_routine_item_response,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    get,
    path = "/api/workout-plans/{workout_plan_id}/next-routine-item",
    params(
        ("workout_plan_id" = Uuid, description = "Workout plan ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
    ),
    responses(
        (status = 200, description = "Next routine item found", body = crate::adapters::http::dtos::workout_plan::WorkoutPlanRoutineItemResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Plans"
}]
pub async fn find_next_routine_item_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(workout_plan_id): Path<Uuid>,
) -> impl IntoResponse {
    match state
        .workout_plan
        .find_next_routine_item
        .execute(current_user, workout_plan_id)
        .await
    {
        Ok(routine_item) => Json(to_routine_item_response(routine_item)).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
