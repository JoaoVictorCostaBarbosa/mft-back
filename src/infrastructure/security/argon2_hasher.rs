use crate::application::errors::CryptoError;
use crate::application::ports::CryptoService;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand::rngs::OsRng;

pub struct Argon2Hasher;

impl CryptoService for Argon2Hasher {
    fn hash(&self, password: &str) -> Result<String, CryptoError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| CryptoError::HashError)?
            .to_string();

        Ok(hash)
    }

    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, CryptoError> {
        let parsed = PasswordHash::new(password_hash).map_err(|_| CryptoError::VerifyError)?;

        let argon = Argon2::default();

        if argon.verify_password(password.as_bytes(), &parsed).is_ok() {
            return Ok(true);
        }

        Ok(false)
    }
}
