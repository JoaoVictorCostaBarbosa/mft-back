use crate::{
    adapters::http::{
        dtos::user_dto::RefreshRequestDTO, errors::http_error::HttpError,
        mappers::user_mapper::UserMappers,
    },
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn refresh_access_handler(
    State(state): State<AppState>,
    Json(token): Json<RefreshRequestDTO>,
) -> impl IntoResponse {
    match state
        .auth
        .refresh_session
        .execute(token.refresh_token)
        .await
    {
        Ok(token) => (
            StatusCode::OK,
            Json(UserMappers::to_refresh_response(token)),
        )
            .into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
