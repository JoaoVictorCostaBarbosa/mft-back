use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::infrastructure::repositories::enums_db::{
    day_of_week_db::DayOfWeekDb, routine_item_type_db::RoutineItemTypeDb,
    routine_mode_db::RoutineModeDb,
};

#[derive(Debug, FromRow)]
pub struct WorkoutPlanRowModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub routine_mode: RoutineModeDb,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
pub struct WorkoutPlanRoutineItemRowModel {
    pub id: Uuid,
    pub item_type: RoutineItemTypeDb,
    pub workout_template_id: Option<Uuid>,
    pub workout_template_user_id: Option<Uuid>,
    pub workout_template_name: Option<String>,
    pub day_of_week: Option<DayOfWeekDb>,
    pub position: Option<i32>,
}
