use crate::application::errors::AppError;
use crate::application::ports::FileStorage;
use crate::domain::commands::UserUpdateFields;
use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use std::sync::Arc;

pub struct UpdateAvatar {
    user_repo: Arc<dyn UserRepository>,
    file_storage: Arc<dyn FileStorage>,
}

impl UpdateAvatar {
    pub fn new(user_repo: Arc<dyn UserRepository>, file_storage: Arc<dyn FileStorage>) -> Self {
        Self {
            user_repo,
            file_storage,
        }
    }

    pub async fn execute(&self, file_img: Vec<u8>, current_user: User) -> Result<User, AppError> {
        if current_user.url_img.is_some() {
            self.file_storage
                .delete_profile_image(current_user.id)
                .await?;
        }

        let new_url = self
            .file_storage
            .upload_profile_image(current_user.id, file_img)
            .await?;

        let updated_user = self
            .user_repo
            .update_user(
                UserUpdateFields {
                    url_img: Some(new_url),
                    ..Default::default()
                },
                current_user.id,
            )
            .await?;

        Ok(updated_user)
    }
}
