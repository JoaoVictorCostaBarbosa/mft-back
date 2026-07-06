use crate::adapters::http::dtos::EquipmentDTO;
use crate::adapters::http::dtos::ExercisePaginationQuery;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::ExerciseMapper;
use crate::application::app_state::AppState;
use crate::application::dtos::exercise::SearchExercisesInput;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path{
    get,
    path = "/api/exercises/equipment/{equipment}",
    params(
        ("equipment" = EquipmentDTO, description = "Exercise equipment"),
        ("page" = Option<u32>, Query, description = "Page number. Defaults to 1"),
        ("per_page" = Option<u32>, Query, description = "Items per page. Defaults to 20, max 100"),
    ),
    responses(
        (status = 200, description = "Exercises found", body = crate::adapters::http::dtos::ExercisePaginatedResponseDTO),
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
    Query(pagination): Query<ExercisePaginationQuery>,
) -> impl IntoResponse {
    match state
        .exercise
        .search
        .execute(
            current_user,
            SearchExercisesInput {
                equipment: Some(equipment.into()),
                muscle_group: None,
                exercise_type: None,
                page: pagination.to_page_request(),
            },
        )
        .await
    {
        Ok(exercises) => {
            let response = ExerciseMapper::paginated_domain_to_response(exercises);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
