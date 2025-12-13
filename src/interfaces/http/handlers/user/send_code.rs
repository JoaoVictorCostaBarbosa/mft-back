use crate::{
    application::app_state::app_state::AppState,
    infrastructure::extractors::current_user::CurrentUser,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

#[utoipa::path{
    post,
    path = "/api/users/send-code",
    responses(
        (status = 200, description = "invited code"),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
}]
pub async fn send_code_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    match state.user.send_change_code.execute(current_user).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "message": "invited code" }))).into_response(),
        Err(e) => e.into_response(),
    }
}
