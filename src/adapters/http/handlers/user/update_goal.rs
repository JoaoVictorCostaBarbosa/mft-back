use crate::adapters::http::dtos::UpdateGoalDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::CurrentUser;
use crate::adapters::http::mappers::UserMappers;
use crate::application::app_state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path{
    patch,
    path = "/api/users/me/goal",
    request_body = UpdateGoalDTO,
    responses(
        (status = 200, description = "Goal updated", body = UserResponseDTO),
        (status = 401, description = "unauthorized"),
        (status = 404, description = "not found"),
        (status = 500, description = "internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Users"
}]
pub async fn update_goal_handler(
    State(state): State<AppState>,
    CurrentUser(current_user): CurrentUser,
    Json(update_request): Json<UpdateGoalDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;

    match state
        .user
        .update_goal
        .execute(update_request.goal.into(), current_user)
        .await
    {
        Ok(user) => {
            let response = mapper.to_user_response_dto(user);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
