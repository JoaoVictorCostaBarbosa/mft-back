use crate::adapters::http::CookieConfig;
use crate::adapters::http::dtos::GoogleLoginRequestDTO;
use crate::adapters::http::errors::HttpError;
use crate::adapters::http::mappers::UserMappers;
use crate::application::app_state::AppState;
use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;

#[utoipa::path{
    post,
    path = "/api/auth/google",
    request_body = GoogleLoginRequestDTO,
    responses(
        (status = 200, description = "google login successful", body = AuthResponseDTO),
        (status = 401, description = "unauthorized"),
        (status = 409, description = "google account conflict"),
        (status = 500, description = "internal server error"),
    ),
    tag = "Auth"
}]
pub async fn google_login_handler(
    State(state): State<AppState>,
    Extension(cookie_config): Extension<CookieConfig>,
    jar: CookieJar,
    Json(request): Json<GoogleLoginRequestDTO>,
) -> impl IntoResponse {
    let mapper = UserMappers;
    let data = mapper.to_google_login_request(request);

    let user = match state.auth.login_with_google.execute(data).await {
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
