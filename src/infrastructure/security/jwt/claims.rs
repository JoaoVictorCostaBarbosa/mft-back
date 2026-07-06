use crate::domain::enums::Role;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub role: RoleClaim,
    pub exp: usize,
}

// Espelho de serialização de Role, como os demais enums (*DTO/*Db):
// mantém o mesmo formato de wire ("Admin"/"User") dos tokens já emitidos.
#[derive(Serialize, Deserialize)]
pub enum RoleClaim {
    Admin,
    User,
}

impl From<Role> for RoleClaim {
    fn from(role: Role) -> Self {
        match role {
            Role::Admin => RoleClaim::Admin,
            Role::User => RoleClaim::User,
        }
    }
}

impl From<RoleClaim> for Role {
    fn from(claim: RoleClaim) -> Self {
        match claim {
            RoleClaim::Admin => Role::Admin,
            RoleClaim::User => Role::User,
        }
    }
}
