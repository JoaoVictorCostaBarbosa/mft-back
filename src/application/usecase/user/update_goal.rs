use crate::application::errors::AppError;
use crate::domain::commands::UserUpdateFields;
use crate::domain::entities::User;
use crate::domain::enums::Goal;
use crate::domain::repositories::UserRepository;
use std::sync::Arc;

pub struct UpdateGoal {
    user_repo: Arc<dyn UserRepository>,
}

impl UpdateGoal {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, goal: Goal, current_user: User) -> Result<User, AppError> {
        let updated_user = self
            .user_repo
            .update_user(
                UserUpdateFields {
                    goal: Some(goal),
                    ..Default::default()
                },
                current_user.id,
            )
            .await?;

        Ok(updated_user)
    }
}
