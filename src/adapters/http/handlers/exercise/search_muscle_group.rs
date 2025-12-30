use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use crate::{
    adapters::http::{
        dtos::{exercise_dto::ExerciseResponse, muscle_group_dto::MuscleGroupDTO},
        errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::exercise_mapper::ExerciseMapper,
    },
    application::app_state::app_state::AppState,
};

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
            let response: Vec<ExerciseResponse> = exercises
                .into_iter()
                .map(|e| ExerciseMapper::domain_to_response(e))
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
