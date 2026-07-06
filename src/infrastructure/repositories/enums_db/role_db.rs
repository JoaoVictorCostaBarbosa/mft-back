use crate::domain::enums::Role;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "role_enum", rename_all = "lowercase")]
pub enum RoleDb {
    Admin,
    User,
}

impl From<RoleDb> for Role {
    fn from(value: RoleDb) -> Self {
        match value {
            RoleDb::Admin => Role::Admin,
            RoleDb::User => Role::User,
        }
    }
}

impl From<Role> for RoleDb {
    fn from(value: Role) -> Self {
        match value {
            Role::Admin => RoleDb::Admin,
            Role::User => RoleDb::User,
        }
    }
}
