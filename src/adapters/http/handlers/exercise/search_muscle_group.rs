use crate::{
    adapters::http::{
        dtos::{exercise_dto::ExerciseResponseDTO, muscle_group_dto::MuscleGroupDTO},
        errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::exercise_mapper::ExerciseMapper,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path{
    get,
    path = "/api/exercises/muscle-group/{muscle_group}",
    params(
        ("muscle_group" = MuscleGroupDTO, description = "Muscle group"),
    ),
    responses(
        (status = 200, description = "Exercises found", body = [ExerciseResponseDTO]),
        (status = 403, description = "denied permission"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Exercises"
}]
pub async fn search_myscle_group_exercise(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(muscle_group): Path<MuscleGroupDTO>,
) -> impl IntoResponse {
    match state
        .exercise
        .search
        .execute(current_user, None, Some(muscle_group.into()), None)
        .await
    {
        Ok(exercises) => {
            let response: Vec<ExerciseResponseDTO> = exercises
                .into_iter()
                .map(|e| ExerciseMapper::domain_to_response(e))
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
