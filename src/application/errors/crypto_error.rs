use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("error genereting hash")]
    HashError,

    #[error("error verifying hash")]
    VerifyError,
}
