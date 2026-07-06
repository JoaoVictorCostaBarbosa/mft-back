#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("failed to upload object: {0}")]
    UploadFailed(String),

    #[error("failed to delete object: {0}")]
    DeleteFailed(String),
}
