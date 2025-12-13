use crate::application::dtos::user::pending_change::PendingChange;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct PendingChangeModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: i32,
    pub limit_date: DateTime<Utc>,
}

impl PendingChangeModel {
    pub fn to(&self) -> PendingChange {
        PendingChange {
            id: self.id,
            user_id: self.user_id,
            code: self.code as u32,
            limit_date: self.limit_date,
        }
    }
}
