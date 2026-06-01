use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct PendingChange {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: u32,
    pub limit_date: DateTime<Utc>,
}

impl PendingChange {
    pub fn new(user_id: Uuid, code: u32) -> Self {
        let datetime = Utc::now() + Duration::minutes(10);

        Self {
            id: Uuid::new_v4(),
            user_id,
            code,
            limit_date: datetime,
        }
    }
}
