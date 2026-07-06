use crate::application::errors::CryptoError;

pub trait RefreshTokenHasher: Send + Sync + 'static {
    fn hash(&self, token: &str) -> Result<String, CryptoError>;
}
