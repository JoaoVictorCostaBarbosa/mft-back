use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

const VERIFICATION_WINDOW_MINUTES: i64 = 10;

#[derive(Debug)]
pub struct PendingUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub code: u32,
    pub limit_date: DateTime<Utc>,
}

impl PendingUser {
    pub fn new(
        name: String,
        email: String,
        password: String,
        code: u32,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            password,
            code,
            limit_date: now + Duration::minutes(VERIFICATION_WINDOW_MINUTES),
        }
    }
}
