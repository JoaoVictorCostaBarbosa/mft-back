use uuid::Uuid;

pub struct WorkoutPlanRequest {
    pub name: String,
}

pub struct WorkoutPlanUpdateRequest {
    pub id: Uuid,
    pub name: Option<String>,
}
