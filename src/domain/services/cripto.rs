use crate::domain::errors::cripto_error::CriptoError;

pub trait CriptoService: Send + Sync + 'static {
    fn hash(&self, password: &str) -> Result<String, CriptoError>;
    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, CriptoError>;
}
