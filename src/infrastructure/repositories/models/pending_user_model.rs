use crate::application::resources::pending_user::PendingUser;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct PendingUserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub code: i32,
    pub limit_date: DateTime<Utc>,
}

impl PendingUserModel {
    pub fn to(&self) -> PendingUser {
        PendingUser {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            code: self.code as u32,
            limit_date: self.limit_date,
        }
    }
}
