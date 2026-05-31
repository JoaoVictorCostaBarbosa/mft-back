use crate::{
    adapters::http::{
        dtos::workout_template::WorkoutTemplateUpdateNameDTO, errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_template_mapper::to_response_workout_template,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    patch,
    path = "/api/workout-templates/change-name",
    request_body = WorkoutTemplateUpdateNameDTO,
    responses(
        (status = 200, description = "Workout template updated", body = WorkoutTemplateResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Templates"
}]
pub async fn update_workout_template_name_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<WorkoutTemplateUpdateNameDTO>,
) -> impl IntoResponse {
    match state
        .workout_template
        .update
        .execute(current_user, request.workout_id, Some(request.name))
        .await
    {
        Ok(wt) => {
            let response = to_response_workout_template(wt);

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
