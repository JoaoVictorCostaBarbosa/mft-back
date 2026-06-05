use crate::domain::enums::set_type::SetType;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "set_type_enum", rename_all = "snake_case")]
pub enum SetTypeDb {
    Warmup,
    Working,
    Drop,
    Failure,
}

impl From<SetTypeDb> for SetType {
    fn from(value: SetTypeDb) -> Self {
        match value {
            SetTypeDb::Warmup => SetType::Warmup,
            SetTypeDb::Working => SetType::Working,
            SetTypeDb::Drop => SetType::Drop,
            SetTypeDb::Failure => SetType::Failure,
        }
    }
}

impl From<SetType> for SetTypeDb {
    fn from(value: SetType) -> Self {
        match value {
            SetType::Warmup => SetTypeDb::Warmup,
            SetType::Working => SetTypeDb::Working,
            SetType::Drop => SetTypeDb::Drop,
            SetType::Failure => SetTypeDb::Failure,
        }
    }
}
