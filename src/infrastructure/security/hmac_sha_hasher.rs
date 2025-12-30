use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::domain::{
    errors::cripto_error::CriptoError, services::refresh_token_hasher::RefreshTokenHasher,
};

type HmacSha256 = Hmac<Sha256>;

pub struct HmacShaHasher {
    secret: String,
}

impl HmacShaHasher {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl RefreshTokenHasher for HmacShaHasher {
    fn hash(&self, token: &str) -> Result<String, CriptoError> {
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .map_err(|_| CriptoError::HashError)?;

        mac.update(token.as_bytes());

        let result = mac.finalize().into_bytes();

        Ok(hex::encode(result))
    }
}
