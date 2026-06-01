use crate::{
    adapters::http::{cookies::ACCESS_TOKEN_COOKIE, errors::http_error::HttpError},
    application::app_state::app_state::AppState,
    domain::{auth::token_data::AccessTokenData, errors::domain_error::DomainError},
};
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
            .ok_or(HttpError(DomainError::Jwt(
                crate::domain::errors::jwt_error::JwtError::MissingClaim,
            )))?;

        let claims = state
            .jwt_service
            .verify_access(token)
            .map_err(|e| HttpError(DomainError::Jwt(e)))?;

        Ok(AuthClaims(claims))
    }
}
