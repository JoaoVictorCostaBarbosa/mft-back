use crate::{
    adapters::http::{
        dtos::{equipment_dto::EquipmentDTO, exercise_dto::ExerciseResponse},
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

pub async fn search_equipment_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(equipment): Path<EquipmentDTO>,
) -> impl IntoResponse {
    match state
        .exercise
        .search
        .execute(current_user, Some(equipment.into()), None, None)
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
