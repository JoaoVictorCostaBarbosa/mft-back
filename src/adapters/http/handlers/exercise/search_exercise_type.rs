use crate::{
    adapters::http::{
        dtos::{exercise_dto::ExercisePaginationQuery, exercise_type_dto::ExerciseTypeDTO},
        errors::http_error::HttpError,
        extractors::current_user::CurrentUser,
        mappers::exercise_mapper::ExerciseMapper,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path{
    get,
    path = "/api/exercises/type/{exercise_type}",
    params(
        ("exercise_type" = ExerciseTypeDTO, description = "Exercise type"),
        ("page" = Option<u32>, Query, description = "Page number. Defaults to 1"),
        ("per_page" = Option<u32>, Query, description = "Items per page. Defaults to 20, max 100"),
    ),
    responses(
        (status = 200, description = "Exercises found", body = crate::adapters::http::dtos::exercise_dto::ExercisePaginatedResponseDTO),
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
    Query(pagination): Query<ExercisePaginationQuery>,
) -> impl IntoResponse {
    match state
        .exercise
        .search
        .execute(
            current_user,
            None,
            None,
            Some(exercise_type.into()),
            pagination.to_pagination_fields(),
        )
        .await
    {
        Ok(exercises) => {
            let result = ExerciseMapper::paginated_domain_to_response(exercises);
            (StatusCode::OK, Json(result)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
