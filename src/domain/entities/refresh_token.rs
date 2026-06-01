use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub hash: String,
    pub expires_at: DateTime<Utc>,
    pub revoked: bool,
    pub created_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn new(user_id: Uuid, hash: String, duration_days: i64) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            user_id,
            hash,
            expires_at: now + Duration::days(duration_days),
            revoked: false,
            created_at: now,
        }
    }
}
