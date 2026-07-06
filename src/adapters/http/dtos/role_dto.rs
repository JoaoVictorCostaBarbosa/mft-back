use crate::domain::enums::Role;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum RoleDTO {
    Admin,
    User,
}

impl From<Role> for RoleDTO {
    fn from(value: Role) -> Self {
        match value {
            Role::Admin => RoleDTO::Admin,
            Role::User => RoleDTO::User,
        }
    }
}
