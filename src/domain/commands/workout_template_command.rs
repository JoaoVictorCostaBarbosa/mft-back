use uuid::Uuid;

#[derive(Debug, Default)]
pub struct WorkoutTemplateFilterFields {
    pub user_id: Uuid,
    pub name: Option<String>,
}
