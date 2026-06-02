use crate::domain::{
    enums::set_type::SetType, errors::set_log_error::SetLogError, value_objects::weight_vo::Weight,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(PartialEq)]
pub struct SetLog {
    pub id: Uuid,
    pub exercise_log_id: Uuid,
    pub set_type: SetType,
    pub weight: Weight,
    pub created_at: DateTime<Utc>,
}

impl SetLog {
    pub fn new(exercise_log_id: Uuid, set_type: SetType, weight: f32) -> Result<Self, SetLogError> {
        Ok(Self {
            id: Uuid::new_v4(),
            exercise_log_id,
            set_type,
            weight: Weight::new(weight)?,
            created_at: Utc::now(),
        })
    }
}
