use crate::{
    adapters::http::{
        cookies::{CookieConfig, REFRESH_TOKEN_COOKIE},
        errors::http_error::HttpError,
        mappers::user_mapper::UserMappers,
    },
    application::app_state::app_state::AppState,
    domain::{errors::domain_error::DomainError, errors::jwt_error::JwtError},
};
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;

#[utoipa::path{
    post,
    path = "/api/auth/refresh",
    responses(
        (status = 200, description = "session refreshed", body = RefreshResponseDTO),
        (status = 401, description = "unauthorized"),
        (status = 500, description = "internal server error"),
    ),
    tag = "Auth"
}]
pub async fn refresh_access_handler(
    State(state): State<AppState>,
    Extension(cookie_config): Extension<CookieConfig>,
    jar: CookieJar,
) -> impl IntoResponse {
    let refresh_token = match jar.get(REFRESH_TOKEN_COOKIE) {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return HttpError(DomainError::Jwt(JwtError::MissingClaim)).into_response();
        }
    };

    match state.auth.refresh_session.execute(refresh_token).await {
        Ok(token) => {
            let jar = cookie_config.add_auth_cookies(jar, token.access, token.refresh);

            (
                StatusCode::OK,
                jar,
                axum::Json(UserMappers::to_refresh_response()),
            )
                .into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
