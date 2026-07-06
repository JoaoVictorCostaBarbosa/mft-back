use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

const VERIFICATION_WINDOW_MINUTES: i64 = 10;

#[derive(Debug)]
pub struct PendingChange {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: u32,
    pub limit_date: DateTime<Utc>,
}

impl PendingChange {
    pub fn new(user_id: Uuid, code: u32, now: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            code,
            limit_date: now + Duration::minutes(VERIFICATION_WINDOW_MINUTES),
        }
    }
}
