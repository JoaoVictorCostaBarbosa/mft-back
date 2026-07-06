use super::claims::AccessClaims;
use crate::application::errors::JwtError;
use crate::application::ports::AccessTokenData;
use crate::application::ports::JwtProvider;
use crate::domain::enums::Role;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

pub struct JwtService {
    access_secret: String,
    access_minutes: i64,
}

impl JwtService {
    pub fn new(access_secret: impl Into<String>, access_minutes: impl Into<i64>) -> Self {
        Self {
            access_secret: access_secret.into(),
            access_minutes: access_minutes.into(),
        }
    }
}

impl JwtProvider for JwtService {
    fn generate_access(&self, user_id: String, role: Role) -> Result<String, JwtError> {
        let exp = (Utc::now() + Duration::minutes(self.access_minutes)).timestamp() as usize;

        let claims = AccessClaims {
            sub: user_id,
            role: role.into(),
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.access_secret.as_bytes()),
        )
        .map_err(JwtError::from)
    }

    fn verify_access(
        &self,
        token: &str,
    ) -> Result<crate::application::ports::AccessTokenData, JwtError> {
        let data = decode::<AccessClaims>(
            token,
            &DecodingKey::from_secret(self.access_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(JwtError::from)?;

        Ok(AccessTokenData {
            user_id: data.claims.sub,
            role: data.claims.role.into(),
        })
    }
}
