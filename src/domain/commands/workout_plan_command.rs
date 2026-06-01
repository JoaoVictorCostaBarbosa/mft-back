use uuid::Uuid;

#[derive(Default)]
pub struct WorkoutPlanFilterFields {
    pub user_id: Uuid,
    pub name: Option<String>,
}
