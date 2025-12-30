use crate::domain::entities::refresh_token::RefreshToken;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct RefreshTokenModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub hash: String,
    pub expires_at: DateTime<Utc>,
    pub revoked: bool,
    pub created_at: DateTime<Utc>,
}

impl From<RefreshToken> for RefreshTokenModel {
    fn from(refresh: RefreshToken) -> Self {
        Self {
            id: refresh.id,
            user_id: refresh.user_id,
            hash: refresh.hash,
            expires_at: refresh.expires_at,
            revoked: refresh.revoked,
            created_at: refresh.created_at,
        }
    }
}

impl From<RefreshTokenModel> for RefreshToken {
    fn from(refresh: RefreshTokenModel) -> Self {
        Self {
            id: refresh.id,
            user_id: refresh.user_id,
            hash: refresh.hash,
            expires_at: refresh.expires_at,
            revoked: refresh.revoked,
            created_at: refresh.created_at,
        }
    }
}
