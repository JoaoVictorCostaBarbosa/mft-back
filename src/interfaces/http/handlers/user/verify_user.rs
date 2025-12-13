use crate::{
    application::app_state::app_state::AppState,
    interfaces::http::{dtos::user_dto::VerifyRequestDTO, mappers::user_mapper::UserMappers},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

#[utoipa::path{
    post,
    path = "/api/auth/verify",
    request_body = VerifyRequestDTO,
    responses(
        (status = 201, description = "create user", body = AuthResponseDTO),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    tag = "Auth"
}]
pub async fn verify_user_handler(
    State(state): State<AppState>,
    Json(verify_data): Json<VerifyRequestDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;

    let data = mapper.to_verify_request(verify_data);

    match state.auth.verify_user.execute(data).await {
        Ok(user) => {
            let user_response = mapper.to_auth_response_dto(user);

            (StatusCode::CREATED, Json(user_response)).into_response()
        }
        Err(err) => err.into_response(),
    }
}
