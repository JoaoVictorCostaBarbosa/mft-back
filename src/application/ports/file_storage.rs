use crate::application::errors::StorageError;
use async_trait::async_trait;
use uuid::Uuid;

/// Armazenamento de arquivos de usuário. A política de path (onde cada tipo
/// de arquivo mora) é decisão da implementação, não de quem chama.
#[async_trait]
pub trait FileStorage: Send + Sync + 'static {
    async fn upload_profile_image(
        &self,
        user_id: Uuid,
        bytes: Vec<u8>,
    ) -> Result<String, StorageError>;
    async fn delete_profile_image(&self, user_id: Uuid) -> Result<(), StorageError>;
}
