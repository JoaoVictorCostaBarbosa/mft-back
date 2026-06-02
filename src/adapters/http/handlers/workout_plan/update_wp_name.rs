use crate::{
    adapters::http::{
        dtos::workout_plan::WorkoutPlanUpdateNameRequestDTO,
        errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_plan_mapper::{
            to_workout_plan_response, to_workout_plan_update_name_request,
        },
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    patch,
    path = "/api/workout-plans/change-name",
    request_body = WorkoutPlanUpdateNameRequestDTO,
    responses(
        (status = 200, description = "Workout plan updated", body = WorkoutPlanResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Plans"
}]
pub async fn update_workout_plan_name_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<WorkoutPlanUpdateNameRequestDTO>,
) -> impl IntoResponse {
    let request = to_workout_plan_update_name_request(request);

    match state
        .workout_plan
        .update
        .execute(current_user, request.id, request.name)
        .await
    {
        Ok(wp) => {
            let response = to_workout_plan_response(wp);

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
