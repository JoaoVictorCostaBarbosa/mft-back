use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use crate::{
    application::app_state::app_state::AppState,
    infrastructure::extractors::current_user::CurrentUser,
    interfaces::http::{dtos::user_dto::UserResponseDTO, mappers::user_mapper::UserMappers},
};

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
        Err(e) => e.into_response(),
    }
}
