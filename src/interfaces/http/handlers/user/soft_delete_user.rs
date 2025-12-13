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

#[utoipa::path{
    patch,
    path = "/api/users/{user_id}/soft-delete",
    params(
        ("user_id" = Uuid, description = "User ID to be deleted", example = "b728b759-4d32-4148-936e-d9036c071d72"),
    ),
    responses(
        (status = 204, description = "no content"),
        (status = 403, description = "denied permission"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
}]
pub async fn soft_delete_user_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    match state
        .user
        .soft_delete_user
        .execute(user_id, current_user)
        .await
    {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => e.into_response(),
    }
}
