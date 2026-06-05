use crate::{
    adapters::http::{
        dtos::workout_plan::UpdateRoutineItemRequestDTO,
        errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_plan_mapper::{
            to_optional_day_of_week, to_optional_routine_item_type, to_routine_item_response,
        },
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
    patch,
    path = "/api/workout-plans/{workout_plan_id}/routine-items/{routine_item_id}",
    request_body = UpdateRoutineItemRequestDTO,
    params(
        ("workout_plan_id" = Uuid, description = "Workout plan ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
        ("routine_item_id" = Uuid, description = "Routine item ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
    ),
    responses(
        (status = 200, description = "Routine item updated", body = crate::adapters::http::dtos::workout_plan::WorkoutPlanRoutineItemResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 409, description = "routine conflict"),
        (status = 422, description = "invalid routine item"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Plans"
}]
pub async fn update_routine_item_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path((workout_plan_id, routine_item_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateRoutineItemRequestDTO>,
) -> impl IntoResponse {
    match state
        .workout_plan
        .update_routine_item
        .execute(
            current_user,
            workout_plan_id,
            routine_item_id,
            to_optional_routine_item_type(request.item_type),
            request.workout_template_id,
            to_optional_day_of_week(request.day_of_week),
            request.position,
        )
        .await
    {
        Ok(routine_item) => Json(to_routine_item_response(routine_item)).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
