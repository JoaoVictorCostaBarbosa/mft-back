use crate::{
    adapters::http::{
        dtos::workout_session::UpdateWorkoutSessionSetRequestDTO,
        errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::workout_session_mapper::{to_set_response, to_set_type},
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
    path = "/api/workout-sessions/{session_id}/sets/{set_id}",
    request_body = UpdateWorkoutSessionSetRequestDTO,
    params(
        ("session_id" = Uuid, description = "Workout session ID"),
        ("set_id" = Uuid, description = "Set ID")
    ),
    responses(
        (status = 200, description = "Set updated", body = crate::adapters::http::dtos::workout_session::WorkoutSessionSetResponseDTO),
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
            session_id,
            set_id,
            to_set_type(request.set_type),
            request.weight,
            request.reps,
        )
        .await
    {
        Ok(set) => Json(to_set_response(set)).into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
