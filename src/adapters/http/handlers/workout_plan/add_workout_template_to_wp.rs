use crate::adapters::http::dtos::AddWorkoutTemplateToPlanRequestDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_optional_day_of_week;
use crate::application::app_state::AppState;
use crate::application::dtos::AddRoutineItemInput;
use crate::domain::enums::RoutineItemType;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    post,
    path = "/api/workout-plans/{workout_plan_id}/workout-template/{workout_template_id}",
    request_body = AddWorkoutTemplateToPlanRequestDTO,
    params(
        ("workout_plan_id" = Uuid, description = "Workout plan ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
        ("workout_template_id" = Uuid, description = "Workout template ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
    ),
    responses(
        (status = 204, description = "Workout template added to workout plan"),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Plans"
}]
pub async fn add_workout_template_to_workout_plan_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path((workout_plan_id, workout_template_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<AddWorkoutTemplateToPlanRequestDTO>,
) -> impl IntoResponse {
    let day_of_week = to_optional_day_of_week(request.day_of_week);

    match state
        .workout_plan
        .add_workout_template
        .execute(
            current_user,
            AddRoutineItemInput {
                workout_plan_id,
                workout_template_id: Some(workout_template_id),
                item_type: RoutineItemType::Workout,
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
