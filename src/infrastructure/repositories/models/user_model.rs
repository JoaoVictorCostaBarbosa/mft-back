use crate::domain::entities::User;
use crate::domain::errors::UserError;
use crate::domain::value_objects::Email;
use crate::domain::value_objects::Name;
use crate::infrastructure::repositories::enums_db::GoalDb;
use crate::infrastructure::repositories::enums_db::RoleDb;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub google_sub: Option<String>,
    pub role: RoleDb,
    pub url_img: Option<String>,
    pub goal: Option<GoalDb>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl UserModel {
    pub fn to_domain(&self) -> Result<User, UserError> {
        Ok(User {
            id: self.id,
            name: Name::new(self.name.clone())?,
            email: Email::new(self.email.clone())?,
            password: self.password.clone(),
            google_sub: self.google_sub.clone(),
            role: self.role.clone().into(),
            url_img: self.url_img.clone(),
            goal: self.goal.map(Into::into),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        })
    }
}
