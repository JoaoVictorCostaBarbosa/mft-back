use crate::{
    adapters::http::{
        dtos::exercise_dto::ExerciseUpdateRequest, errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
    },
    application::app_state::app_state::AppState,
    domain::commands::exercise_commands::ExerciseUpdateFields,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    put,
    path = "/api/exercises",
    request_body = ExerciseUpdateRequest,
    responses(
        (status = 204, description = "Exercise updated"),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Exercises"
}]
pub async fn update_exercise_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(data): Json<ExerciseUpdateRequest>,
) -> impl IntoResponse {
    let fields = ExerciseUpdateFields {
        id: data.id,
        name: data.name,
        exercise_type: data.exercise_type.map(Into::into),
        equipment: data.equipment.map(Into::into),
        muscle_group: data.muscle_group.map(Into::into),
    };

    match state.exercise.update.execute(current_user, fields).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
