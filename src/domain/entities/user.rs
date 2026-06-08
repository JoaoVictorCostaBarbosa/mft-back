use crate::domain::{
    enums::role::Role,
    errors::user_error::UserError,
    value_objects::{email_vo::Email, name_vo::Name},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: Name,
    pub email: Email,
    pub password: String,
    pub google_sub: Option<String>,
    pub role: Role,
    pub url_img: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(name: String, email: String, password_hash: String) -> Result<Self, UserError> {
        let now: DateTime<Utc> = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            name: Name::new(name)?,
            email: Email::new(email)?,
            password: password_hash,
            google_sub: None,
            role: Role::User,
            url_img: None,
            created_at: now,
            updated_at: None,
            deleted_at: None,
        })
    }
}
