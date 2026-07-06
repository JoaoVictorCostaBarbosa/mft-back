use crate::adapters::http::dtos::UpdateWorkoutSessionSetRequestDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::to_set_response;
use crate::adapters::http::mappers::to_set_type;
use crate::application::app_state::AppState;
use crate::application::dtos::workout_session::UpdateSessionSetInput;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path{
    patch,
    path = "/api/workout-sessions/{session_id}/sets/{set_id}",
    request_body = UpdateWorkoutSessionSetRequestDTO,
    params(
        ("session_id" = Uuid, description = "Workout session ID"),
        ("set_id" = Uuid, description = "Set ID")
    ),
    responses(
        (status = 200, description = "Set updated", body = crate::adapters::http::dtos::WorkoutSessionSetResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 409, description = "session is not editable"),
        (status = 422, description = "invalid set"),
        (status = 500, description = "internal server error"),
    ),
    security(("bearer_auth" = [])),
    tag = "Workout Sessions"
}]
pub async fn update_workout_session_set_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path((session_id, set_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateWorkoutSessionSetRequestDTO>,
) -> impl IntoResponse {
    match state
        .workout_session
        .update_set
        .execute(
            current_user,
            UpdateSessionSetInput {
                session_id,
                set_id,
                set_type: to_set_type(request.set_type),
                weight: request.weight,
                reps: request.reps,
            },
        )
        .await
    {
        Ok(set) => Json(to_set_response(set)).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
