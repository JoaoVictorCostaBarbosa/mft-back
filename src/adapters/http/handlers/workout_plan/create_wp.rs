use crate::{
    adapters::http::{
        dtos::workout_plan::WorkoutPlanRequestDTO,
        errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_plan_mapper::{to_workout_plan_request, to_workout_plan_response},
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    post,
    path = "/api/workout-plans",
    request_body = WorkoutPlanRequestDTO,
    responses(
        (status = 201, description = "Workout plan created", body = WorkoutPlanResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Plans"
}]
pub async fn create_workout_plan_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<WorkoutPlanRequestDTO>,
) -> impl IntoResponse {
    let request = to_workout_plan_request(request);

    match state
        .workout_plan
        .create
        .execute(current_user, request)
        .await
    {
        Ok(wp) => {
            let response = to_workout_plan_response(wp);

            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
