use crate::adapters::http::dtos::GoalDTO;
use crate::adapters::http::dtos::RoleDTO;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequestDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequestDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GoogleLoginRequestDTO {
    pub id_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponseDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: RoleDTO,
    pub url_img: Option<String>,
    pub goal: Option<GoalDTO>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponseDTO {
    pub user: UserResponseDTO,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyRequestDTO {
    pub email: String,
    pub code: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserDTO {
    pub id: Option<Uuid>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEmailDTO {
    pub id: Option<Uuid>,
    pub email: String,
    pub code: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePasswordDTO {
    pub id: Option<Uuid>,
    pub password: String,
    pub code: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateGoalDTO {
    pub goal: GoalDTO,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshResponseDTO {}
