use crate::domain::{
    commands::user_commands::UserUpdateFilds, entities::user::User,
    errors::domain_error::DomainError, repositories::user_repository::UserRepository,
    services::bucket_storage::BucketStorage,
};
use std::sync::Arc;

pub struct UpdateAvatar {
    pub user_repo: Arc<dyn UserRepository>,
    pub bucket_service: Arc<dyn BucketStorage>,
}

impl UpdateAvatar {
    pub fn new(user_repo: Arc<dyn UserRepository>, bucket_service: Arc<dyn BucketStorage>) -> Self {
        Self {
            user_repo,
            bucket_service,
        }
    }

    pub async fn execute(
        &self,
        file_img: Vec<u8>,
        current_user: User,
    ) -> Result<User, DomainError> {
        let path = format!("users/{}/profile", current_user.id);

        if current_user.url_img.is_some() {
            // Depois ver um jeito melhor para isso, pois se o path mudar,
            // o delete não vai funcionar para url's antigas
            self.bucket_service.delete_file(&path).await?;
        }

        let new_url = self.bucket_service.upload_file(&path, file_img).await?;

        let updated_user = self
            .user_repo
            .update_user(
                UserUpdateFilds {
                    url_img: Some(new_url),
                    ..Default::default()
                },
                current_user.id,
            )
            .await?;

        Ok(updated_user)
    }
}
