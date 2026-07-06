use crate::application::errors::JwtError;
use crate::domain::enums::Role;

#[derive(Debug, Clone)]
pub struct AccessTokenData {
    pub user_id: String,
    pub role: Role,
}

pub trait JwtProvider: Send + Sync + 'static {
    fn generate_access(&self, user_id: String, role: Role) -> Result<String, JwtError>;
    fn verify_access(&self, token: &str) -> Result<AccessTokenData, JwtError>;
}
