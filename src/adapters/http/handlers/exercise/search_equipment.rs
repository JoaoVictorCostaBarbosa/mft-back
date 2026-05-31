use crate::{
    adapters::http::{
        dtos::{equipment_dto::EquipmentDTO, exercise_dto::ExerciseResponseDTO},
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
    path = "/api/exercises/equipment/{equipment}",
    params(
        ("equipment" = EquipmentDTO, description = "Exercise equipment"),
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
            let response: Vec<ExerciseResponseDTO> = exercises
                .into_iter()
                .map(|e| ExerciseMapper::domain_to_response(e))
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
