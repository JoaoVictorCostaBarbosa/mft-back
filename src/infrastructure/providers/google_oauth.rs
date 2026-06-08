use crate::domain::{
    errors::{
        domain_error::DomainError, permission_error::PermissionError,
        repository_error::RepositoryError,
    },
    services::google_oauth::{GoogleOAuthProvider, GoogleUserInfo},
};
use axum::async_trait;
use reqwest::Client;
use serde::Deserialize;

pub struct GoogleOAuthHttpProvider {
    client: Client,
    client_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleTokenInfoResponse {
    sub: String,
    aud: String,
    email: String,
    email_verified: GoogleEmailVerified,
    name: Option<String>,
    picture: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GoogleEmailVerified {
    Bool(bool),
    String(String),
}

impl GoogleEmailVerified {
    fn is_verified(&self) -> bool {
        match self {
            GoogleEmailVerified::Bool(value) => *value,
            GoogleEmailVerified::String(value) => value == "true",
        }
    }
}

impl GoogleOAuthHttpProvider {
    pub fn new(client_id: Option<String>) -> Self {
        Self {
            client: Client::new(),
            client_id,
        }
    }
}

#[async_trait]
impl GoogleOAuthProvider for GoogleOAuthHttpProvider {
    async fn verify_id_token(&self, id_token: &str) -> Result<GoogleUserInfo, DomainError> {
        let client_id = self.client_id.as_ref().ok_or_else(|| {
            DomainError::Repository(RepositoryError::Unexpected(
                "GOOGLE_CLIENT_ID is not configured".to_string(),
            ))
        })?;

        let token_info = self
            .client
            .get("https://oauth2.googleapis.com/tokeninfo")
            .query(&[("id_token", id_token)])
            .send()
            .await
            .map_err(|e| RepositoryError::Unexpected(format!("google token request error: {e}")))?
            .error_for_status()
            .map_err(|_| PermissionError::Unauthorized)?
            .json::<GoogleTokenInfoResponse>()
            .await
            .map_err(|e| RepositoryError::Unexpected(format!("google token parse error: {e}")))?;

        if token_info.aud != *client_id || !token_info.email_verified.is_verified() {
            return Err(PermissionError::Unauthorized.into());
        }

        let name = token_info.name.unwrap_or_else(|| {
            token_info
                .email
                .split('@')
                .next()
                .unwrap_or("User")
                .to_string()
        });

        Ok(GoogleUserInfo {
            sub: token_info.sub,
            email: token_info.email,
            name,
            picture: token_info.picture,
        })
    }
}
