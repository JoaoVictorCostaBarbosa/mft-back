use crate::{
    adapters::http::{
        dtos::user_dto::UpdatePasswordDTO, errors::http_error::HttpError,
        extractors::current_user::CurrentUser, mappers::user_mapper::UserMappers,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    patch,
    path = "/api/users/me/password",
    request_body = UpdatePasswordDTO,
    responses(
        (status = 204, description = "Password updated"),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
}]
pub async fn update_password_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(user_request): Json<UpdatePasswordDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;
    let request = mapper.to_update_password_request(user_request);

    match state
        .user
        .change_password
        .execute(request, current_user)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
