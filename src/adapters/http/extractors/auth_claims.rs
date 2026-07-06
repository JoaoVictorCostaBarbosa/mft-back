use crate::adapters::http::ACCESS_TOKEN_COOKIE;
use crate::adapters::http::errors::HttpError;
use crate::application::app_state::AppState;
use crate::application::errors::AppError;
use crate::application::ports::AccessTokenData;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::cookie::CookieJar;

pub struct AuthClaims(pub AccessTokenData);

#[async_trait]
impl FromRequestParts<AppState> for AuthClaims {
    type Rejection = HttpError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let token = jar
            .get(ACCESS_TOKEN_COOKIE)
            .map(|cookie| cookie.value())
            .ok_or(HttpError(AppError::Jwt(
                crate::application::errors::JwtError::MissingClaim,
            )))?;

        let claims = state
            .jwt_service
            .verify_access(token)
            .map_err(|e| HttpError(AppError::Jwt(e)))?;

        Ok(AuthClaims(claims))
    }
}
