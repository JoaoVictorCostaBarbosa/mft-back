use crate::application::errors::CryptoError;

pub trait CryptoService: Send + Sync + 'static {
    fn hash(&self, password: &str) -> Result<String, CryptoError>;
    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, CryptoError>;
}
