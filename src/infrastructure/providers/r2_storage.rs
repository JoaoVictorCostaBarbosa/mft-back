use crate::domain::{errors::bucket_error::BucketError, services::bucket_storage::BucketStorage};
use aws_sdk_s3::{Client, config::{Region, BehaviorVersion}};
use axum::async_trait;

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
            .behavior_version(BehaviorVersion::latest())   // ⭐ OBRIGATÓRIO
            .build();

        let client = Client::from_conf(config);

        Self {
            client,
            bucket_name: bucket_name.to_string(),
            public_base_url: public_base_url.to_string(),
        }
    }
}

#[async_trait]
impl BucketStorage for R2Storage {
    async fn upload_file(&self, path: &str, bytes: Vec<u8>) -> Result<String, BucketError> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(path)
            .body(bytes.into())
            .send()
            .await
            .map_err(|err| BucketError::UploadFailed(err.to_string()))?;

        Ok(format!("{}/{}", self.public_base_url, path))
    }

    async fn delete_file(&self, path: &str) -> Result<(), BucketError> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(path)
            .send()
            .await
            .map_err(|err| BucketError::DeleteFailed(err.to_string()))?;

        Ok(())
    }
}
