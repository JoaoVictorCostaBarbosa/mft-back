use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{application::app_state::app_state::AppState, infrastructure::extractors::current_user::CurrentUser, interfaces::http::{dtos::user_dto::UpdatePasswordDTO, mappers::user_mapper::UserMappers}};

pub async fn update_password_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(user_request): Json<UpdatePasswordDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;
    let request = mapper.to_update_password_request(user_request);
    
    match state.user.change_password.execute(request, current_user).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => e.into_response(),
    }
}