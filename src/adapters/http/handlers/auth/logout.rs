use crate::{
    adapters::http::{dtos::user_dto::RefreshRequestDTO, errors::http_error::HttpError},
    application::app_state::app_state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn logout_handler(
    State(state): State<AppState>,
    Json(token): Json<RefreshRequestDTO>,
) -> impl IntoResponse {
    match state.auth.logout.execute(token.refresh_token).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
