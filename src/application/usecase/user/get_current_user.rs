use crate::domain::entities::User;

pub struct GetCurrentUser;

impl GetCurrentUser {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, current_user: User) -> User {
        current_user
    }
}
