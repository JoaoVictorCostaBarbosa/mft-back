use crate::{
    application::app_state::app_state::AppState,
    infrastructure::extractors::current_user::CurrentUser,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn delete_user_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.user.delete_user.execute(id, current_user).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => e.into_response(),
    }
}
