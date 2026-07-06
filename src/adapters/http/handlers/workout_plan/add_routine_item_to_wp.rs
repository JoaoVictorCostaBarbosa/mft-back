use crate::adapters::http::dtos::AddRoutineItemToPlanRequestDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_optional_day_of_week;
use crate::adapters::http::mappers::to_routine_item_type;
use crate::application::app_state::AppState;
use crate::application::dtos::AddRoutineItemInput;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    post,
    path = "/api/workout-plans/{workout_plan_id}/routine-items",
    request_body = AddRoutineItemToPlanRequestDTO,
    params(
        ("workout_plan_id" = Uuid, description = "Workout plan ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
    ),
    responses(
        (status = 204, description = "Routine item added to workout plan"),
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
pub async fn add_routine_item_to_workout_plan_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(workout_plan_id): Path<Uuid>,
    Json(request): Json<AddRoutineItemToPlanRequestDTO>,
) -> impl IntoResponse {
    let item_type = to_routine_item_type(request.item_type);
    let day_of_week = to_optional_day_of_week(request.day_of_week);

    match state
        .workout_plan
        .add_workout_template
        .execute(
            current_user,
            AddRoutineItemInput {
                workout_plan_id,
                workout_template_id: request.workout_template_id,
                item_type,
                day_of_week,
                position: request.position,
            },
        )
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
