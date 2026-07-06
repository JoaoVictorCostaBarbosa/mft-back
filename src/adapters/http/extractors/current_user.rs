use crate::adapters::http::errors::HttpError;
use crate::adapters::http::extractors::AuthClaims;
use crate::application::app_state::AppState;
use crate::application::errors::AppError;
use crate::application::errors::JwtError;
use crate::domain::entities::User;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

pub struct CurrentUser(pub User);

#[async_trait]
impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = HttpError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let AuthClaims(claims) = AuthClaims::from_request_parts(parts, state).await?;

        let user_id = Uuid::parse_str(claims.user_id.as_str()).map_err(|_| {
            HttpError(AppError::Jwt(JwtError::Internal(
                "id in token is invalid".to_string(),
            )))
        })?;

        let user = state
            .auth
            .get_authenticated_user
            .execute(user_id)
            .await
            .map_err(HttpError)?;

        Ok(CurrentUser(user))
    }
}
