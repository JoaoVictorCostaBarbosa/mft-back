use crate::domain::{
    auth::token_data::AccessTokenData, enums::role::Role, errors::jwt_error::JwtError,
};

pub trait JwtProvider: Send + Sync + 'static {
    fn generate_access(&self, user_id: String, role: Role) -> Result<String, JwtError>;
    fn verify_access(&self, token: &str) -> Result<AccessTokenData, JwtError>;
}
