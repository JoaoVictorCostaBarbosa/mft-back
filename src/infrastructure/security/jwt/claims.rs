use crate::domain::enums::role::Role;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub role: Role,
    pub exp: usize,
}
