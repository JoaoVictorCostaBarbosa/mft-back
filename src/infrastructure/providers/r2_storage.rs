use crate::application::errors::StorageError;
use crate::application::ports::FileStorage;
use async_trait::async_trait;
use aws_sdk_s3::{
    Client,
    config::{BehaviorVersion, Region},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct R2Storage {
    client: Client,
    bucket_name: String,
    public_base_url: String,
}

impl R2Storage {
    pub fn new(
        access_key: &str,
        secret_key: &str,
        bucket_name: &str,
        public_base_url: &str,
        s3_endpoint: &str,
    ) -> Self {
        let credentials =
            aws_sdk_s3::config::Credentials::new(access_key, secret_key, None, None, "r2");

        let config = aws_sdk_s3::config::Builder::new()
            .region(Region::new("auto"))
            .credentials_provider(credentials)
            .endpoint_url(s3_endpoint)
            .force_path_style(true)
            .behavior_version(BehaviorVersion::latest()) // ⭐ OBRIGATÓRIO
            .build();

        let client = Client::from_conf(config);

        Self {
            client,
            bucket_name: bucket_name.to_string(),
            public_base_url: public_base_url.to_string(),
        }
    }
}

fn profile_image_path(user_id: Uuid) -> String {
    format!("users/{user_id}/profile")
}

fn image_content_type(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(&[0x89, b'P', b'N', b'G']) {
        "image/png"
    } else {
        "image/jpeg"
    }
}

#[async_trait]
impl FileStorage for R2Storage {
    async fn upload_profile_image(
        &self,
        user_id: Uuid,
        bytes: Vec<u8>,
    ) -> Result<String, StorageError> {
        let path = profile_image_path(user_id);
        let content_type = image_content_type(&bytes);
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&path)
            .content_type(content_type)
            .body(bytes.into())
            .send()
            .await
            .map_err(|err| StorageError::UploadFailed(err.to_string()))?;

        // A chave no bucket é fixa por usuário; o ?v= muda a URL a cada troca
        // para o navegador/CDN não servirem a foto anterior do cache.
        let version = chrono::Utc::now().timestamp();
        Ok(format!("{}/{}?v={}", self.public_base_url, path, version))
    }

    async fn delete_profile_image(&self, user_id: Uuid) -> Result<(), StorageError> {
        let path = profile_image_path(user_id);
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(&path)
            .send()
            .await
            .map_err(|err| StorageError::DeleteFailed(err.to_string()))?;

        Ok(())
    }
}
