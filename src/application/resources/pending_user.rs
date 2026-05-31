use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

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
    pub fn new(name: String, email: String, password: String, code: u32) -> Self {
        let datetime = Utc::now() + Duration::minutes(10);

        Self {
            id: Uuid::new_v4(),
            name,
            email,
            password,
            code,
            limit_date: datetime,
        }
    }
}
