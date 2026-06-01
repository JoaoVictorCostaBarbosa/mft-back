use crate::{
    adapters::http::{
        cookies::CookieConfig, dtos::user_dto::VerifyRequestDTO, errors::http_error::HttpError,
        mappers::user_mapper::UserMappers,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;

#[utoipa::path{
    post,
    path = "/api/auth/verify",
    request_body = VerifyRequestDTO,
    responses(
        (status = 200, description = "create user", body = AuthResponseDTO),
        (status = 422, description = "unprocessable entity"),
        (status = 500, description = "internal server error"),
    ),
    tag = "Auth"
}]
pub async fn verify_user_handler(
    State(state): State<AppState>,
    Extension(cookie_config): Extension<CookieConfig>,
    jar: CookieJar,
    Json(request): Json<VerifyRequestDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;
    let data = mapper.to_verify_request(request);

    let user = match state.auth.verify_user.execute(data).await {
        Ok(user) => user,
        Err(e) => return HttpError(e).into_response(),
    };

    let access = match state
        .jwt_service
        .generate_access(user.id.to_string(), user.role)
    {
        Ok(token) => token,
        Err(e) => return HttpError(e.into()).into_response(),
    };

    let refresh = match state.auth.issue_token_service.execute(user.id).await {
        Ok(token) => token,
        Err(e) => return HttpError(e.into()).into_response(),
    };

    let response = mapper.to_auth_response_dto(user);
    let jar = cookie_config.add_auth_cookies(jar, access, refresh);

    (StatusCode::OK, jar, Json(response)).into_response()
}
