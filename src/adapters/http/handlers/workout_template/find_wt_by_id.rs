use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_response_workout_template;
use crate::application::app_state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    get,
    path = "/api/workout-templates/{workout_id}",
    params(
        ("workout_id" = Uuid, description = "Workout template ID", example = "b728b759-4d32-4148-936e-d9036c071d72"),
    ),
    responses(
        (status = 200, description = "Workout template found", body = WorkoutTemplateResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Workout Templates"
}]
pub async fn find_workout_template_by_id_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(workout_id): Path<Uuid>,
) -> impl IntoResponse {
    match state
        .workout_template
        .find_by_id
        .execute(current_user, workout_id)
        .await
    {
        Ok(wt) => {
            let response = to_response_workout_template(wt);

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
