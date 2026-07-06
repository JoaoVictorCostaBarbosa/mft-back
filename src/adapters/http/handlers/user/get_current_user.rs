use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::UserMappers;
use crate::application::app_state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "Current user found", body = crate::adapters::http::dtos::UserResponseDTO),
        (status = 401, description = "unauthorized"),
        (status = 403, description = "denied permission"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
}]
pub async fn get_current_user_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    let mapper = UserMappers;
    let user = state.user.get_current_user.execute(current_user).await;
    let response = mapper.to_user_response_dto(user);

    (StatusCode::OK, Json(response)).into_response()
}
