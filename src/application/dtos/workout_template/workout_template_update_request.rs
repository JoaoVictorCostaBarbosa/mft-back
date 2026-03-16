use uuid::Uuid;

pub struct WorkoutTemplateUpdateRequest {
    pub workout_id: Uuid,
    pub name: Option<String>,
}
