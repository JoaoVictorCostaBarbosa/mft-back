use crate::adapters::http::dtos::UpdateUserDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::UserMappers;
use crate::application::app_state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path{
    patch,
    path = "/api/users",
    request_body = UpdateUserDTO,
    responses(
        (status = 200, description = "User updated", body = UserResponseDTO),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
}]
pub async fn update_user_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(update_request): Json<UpdateUserDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;
    let user_data = mapper.to_update_user_request(update_request);

    match state
        .user
        .update_user
        .execute(user_data, current_user)
        .await
    {
        Ok(user) => {
            let response = mapper.to_user_response_dto(user);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
