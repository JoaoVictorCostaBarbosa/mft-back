use crate::{
    application::app_state::app_state::AppState,
    domain::{
        entities::user::User,
        errors::{domain_error::DomainError, jwt_error::JwtError, permission_error::PermissionError, repository_error::RepositoryError},
    },
    infrastructure::extractors::auth_claims::AuthClaims,
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

pub struct CurrentUser(pub User);

#[async_trait]
impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = DomainError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState
    ) -> Result<Self, Self::Rejection> {
        let AuthClaims(claims) = AuthClaims::from_request_parts(parts, state).await?;

        let user_id = Uuid::parse_str(claims.user_id.as_str()).map_err(|_| {
            DomainError::Jwt(JwtError::Internal("id in token is invalid".to_string()))
        })?;

        let user = state.auth.user_repo.get_user_by_id(user_id).await;

        match user {
            Ok(u) => {
                if u.deleted_at.is_some() {
                    return Err(DomainError::Permisson(PermissionError::Forbidden));
                }
                
                Ok(CurrentUser(u))
            }
            Err(DomainError::Repository(RepositoryError::NotFound(_))) => {
                Err(DomainError::Permisson(PermissionError::Forbidden))
            }
            Err(e) => Err(e),
        }
    }
}
