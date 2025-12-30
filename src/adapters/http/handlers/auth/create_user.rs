use crate::{
    adapters::http::{
        dtos::user_dto::CreateUserRequestDTO, errors::http_error::HttpError,
        mappers::user_mapper::UserMappers,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    extract::{Json, State},
    http::status,
    response::IntoResponse,
};
use serde_json::json;

#[utoipa::path{
    post,
    path = "/api/auth/register",
    request_body = CreateUserRequestDTO,
    responses(
        (status = 200, description = "code sent"),
        (status = 409, description = "email already used"),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    tag = "Auth"
}]
pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(user_data): Json<CreateUserRequestDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;
    let user_create_dto = mapper.to_user_create_dto(user_data);

    match state.auth.create_user.execute(user_create_dto).await {
        Ok(_) => (
            status::StatusCode::OK,
            Json(json!({ "message": "invited code" })),
        )
            .into_response(),
        Err(e) => HttpError(e).into_response(),
    }
}
