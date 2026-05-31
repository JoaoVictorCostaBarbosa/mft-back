use crate::{
    adapters::http::{
        errors::http_error::HttpError,
        extractors::{current_user::CurrentUser, image_file::ImageFile},
        mappers::user_mapper::UserMappers,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

#[utoipa::path{
    patch,
    path = "/api/users/me/avatar",
    request_body(content = String, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Avatar updated", body = UserResponseDTO),
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
pub async fn update_avatar_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    ImageFile(file): ImageFile,
) -> impl IntoResponse {
    let mapper = UserMappers;

    match state.user.update_avatar.execute(file, current_user).await {
        Ok(user) => {
            let response = mapper.to_user_response_dto(user);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
