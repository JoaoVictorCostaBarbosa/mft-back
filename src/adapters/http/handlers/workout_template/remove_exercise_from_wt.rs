use crate::{
    adapters::http::{
        dtos::workout_template::WorkoutTemplateExerciseDTO, errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_template_mapper::to_request_workout_template_exercise,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    post,
    path = "/api/workout-templates/remove-exercise",
    request_body = WorkoutTemplateExerciseDTO,
    responses(
        (status = 204, description = "Exercise removed from workout template"),
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
pub async fn remove_exercise_from_workout_template_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(request): Json<WorkoutTemplateExerciseDTO>,
) -> impl IntoResponse {
    let wte = to_request_workout_template_exercise(request);

    match state
        .workout_template
        .remove_exercise
        .execute(current_user, wte)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
