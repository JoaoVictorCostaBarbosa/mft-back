use crate::adapters::http::dtos::WorkoutTemplateRequestDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_request_workout_template;
use crate::adapters::http::mappers::to_response_workout_template;
use crate::application::app_state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    post,
    path = "/api/workout-templates",
    request_body = WorkoutTemplateRequestDTO,
    responses(
        (status = 201, description = "Workout template created", body = WorkoutTemplateResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Templates"
}]
pub async fn create_workout_template_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<WorkoutTemplateRequestDTO>,
) -> impl IntoResponse {
    let wt = to_request_workout_template(request);

    match state
        .workout_template
        .create
        .execute(current_user, wt)
        .await
    {
        Ok(template) => {
            let response = to_response_workout_template(template);

            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
