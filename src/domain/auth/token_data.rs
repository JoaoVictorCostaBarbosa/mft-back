use crate::domain::enums::role::Role;

#[derive(Debug, Clone)]
pub struct AccessTokenData {
    pub user_id: String,
    pub role: Role,
}

#[derive(Debug, Clone)]
pub struct RefreshTokenData {
    pub user_id: String,
}
