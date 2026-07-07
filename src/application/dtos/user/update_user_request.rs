use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UpdateUserRequest {
    pub id: Option<Uuid>,
    pub name: Option<String>,
}
