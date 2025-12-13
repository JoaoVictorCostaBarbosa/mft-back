use crate::{
    application::app_state::app_state::AppState,
    infrastructure::extractors::current_user::CurrentUser,
    interfaces::http::{dtos::user_dto::UpdateEmailDTO, mappers::user_mapper::UserMappers},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

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
        Err(e) => e.into_response(),
    }
}
