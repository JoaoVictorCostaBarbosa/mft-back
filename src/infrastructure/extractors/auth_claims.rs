use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use crate::{
    domain::{
        errors::domain_error::DomainError,
        auth::token_data::AccessTokenData,
    },
    application::app_state::app_state::AppState,
};

pub struct AuthClaims(pub AccessTokenData);

#[async_trait]
impl FromRequestParts<AppState> for AuthClaims {
    type Rejection = DomainError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(DomainError::Jwt(crate::domain::errors::jwt_error::JwtError::MissingClaim))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(DomainError::Jwt(crate::domain::errors::jwt_error::JwtError::MissingClaim));
        }

        let token = auth_header.trim_start_matches("Bearer ").trim();

        let claims = state
            .jwt_service
            .verify_access(token)
            ?; 

        Ok(AuthClaims(claims))
    }
}
