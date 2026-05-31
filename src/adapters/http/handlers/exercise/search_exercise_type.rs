use crate::{
    adapters::http::{
        dtos::{exercise_dto::ExerciseResponseDTO, exercise_type_dto::ExerciseTypeDTO},
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
    path = "/api/exercises/type/{exercise_type}",
    params(
        ("exercise_type" = ExerciseTypeDTO, description = "Exercise type"),
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
pub async fn search_exercise_type_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(exercise_type): Path<ExerciseTypeDTO>,
) -> impl IntoResponse {
    match state
        .exercise
        .search
        .execute(current_user, None, None, Some(exercise_type.into()))
        .await
    {
        Ok(exercises) => {
            let result: Vec<ExerciseResponseDTO> = exercises
                .into_iter()
                .map(|e| ExerciseMapper::domain_to_response(e))
                .collect();
            (StatusCode::OK, Json(result)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
