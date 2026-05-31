use crate::{
    adapters::http::{
        dtos::user_dto::UserResponseDTO, errors::http_error::HttpError,
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
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "Users found", body = [UserResponseDTO]),
        (status = 403, description = "denied permission"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
}]
pub async fn find_users_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    let mapper = UserMappers;

    match state.user.find_users.execute(current_user).await {
        Ok(users) => {
            let users_response: Vec<UserResponseDTO> = users
                .into_iter()
                .map(|u| mapper.to_user_response_dto(u))
                .collect();
            (StatusCode::OK, Json(users_response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
