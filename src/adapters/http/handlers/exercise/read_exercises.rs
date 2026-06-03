use crate::{
    adapters::http::{
        dtos::exercise_dto::ExercisePaginationQuery, errors::http_error::HttpError,
        extractors::current_user::CurrentUser, mappers::exercise_mapper::ExerciseMapper,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path{
    get,
    path = "/api/exercises",
    params(
        ("page" = Option<u32>, Query, description = "Page number. Defaults to 1"),
        ("per_page" = Option<u32>, Query, description = "Items per page. Defaults to 20, max 100"),
    ),
    responses(
        (status = 200, description = "Exercises found", body = crate::adapters::http::dtos::exercise_dto::ExercisePaginatedResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Exercises"
}]
pub async fn read_exercises_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Query(pagination): Query<ExercisePaginationQuery>,
) -> impl IntoResponse {
    match state
        .exercise
        .read
        .execute(current_user, pagination.to_pagination_fields())
        .await
    {
        Ok(exercises) => {
            let response = ExerciseMapper::paginated_domain_to_response(exercises);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
