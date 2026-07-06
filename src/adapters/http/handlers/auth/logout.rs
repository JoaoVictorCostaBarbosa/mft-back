use crate::adapters::http::CookieConfig;
use crate::adapters::http::REFRESH_TOKEN_COOKIE;
use crate::adapters::http::errors::HttpError;
use crate::application::app_state::AppState;
use crate::application::errors::AppError;
use crate::application::errors::JwtError;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;

#[utoipa::path{
    patch,
    path = "/api/auth/logout",
    responses(
        (status = 200, description = "logout successful"),
        (status = 401, description = "unauthorized"),
        (status = 500, description = "internal server error"),
    ),
    tag = "Auth"
}]
pub async fn logout_handler(
    State(state): State<AppState>,
    Extension(cookie_config): Extension<CookieConfig>,
    jar: CookieJar,
) -> impl IntoResponse {
    let refresh_token = match jar.get(REFRESH_TOKEN_COOKIE) {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return HttpError(AppError::Jwt(JwtError::MissingClaim)).into_response();
        }
    };

    match state.auth.logout.execute(refresh_token).await {
        Ok(_) => {
            let jar = cookie_config.clear_auth_cookies(jar);

            (StatusCode::OK, jar).into_response()
        }
        Err(e) => HttpError(e).into_response(),
    }
}
