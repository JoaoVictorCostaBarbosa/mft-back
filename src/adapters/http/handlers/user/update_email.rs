use crate::{
    adapters::http::{
        dtos::user_dto::UpdateEmailDTO, errors::http_error::HttpError,
        extractors::current_user::CurrentUser, mappers::user_mapper::UserMappers,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path{
    patch,
    path = "/api/users/me/email",
    request_body = UpdateEmailDTO,
    responses(
        (status = 200, description = "Email updated", body = UserResponseDTO),
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
pub async fn update_email_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(user_request): Json<UpdateEmailDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;
    let user_data = mapper.to_update_email_request(user_request);

    match state
        .user
        .change_email
        .execute(user_data, current_user)
        .await
    {
        Ok(e) => {
            let response = mapper.to_user_response_dto(e);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
