use crate::adapters::http::dtos::*;
use crate::application::dtos::auth::GoogleLoginRequest;
use crate::application::dtos::auth::LoginRequest;
use crate::application::dtos::auth::UserCreate;
use crate::application::dtos::auth::VerifyRequest;
use crate::application::dtos::user::EmailChangeRequest;
use crate::application::dtos::user::PasswordChangeRequest;
use crate::application::dtos::user::UpdateUserRequest;
use crate::domain::entities::User;

pub struct UserMappers;

impl UserMappers {
    pub fn to_user_create_dto(&self, request: CreateUserRequestDTO) -> UserCreate {
        UserCreate {
            name: request.name,
            email: request.email,
            password: request.password,
        }
    }

    pub fn to_user_response_dto(&self, response: User) -> UserResponseDTO {
        UserResponseDTO {
            id: response.id,
            name: response.name.value().to_string(),
            email: response.email.clone().value().to_string(),
            role: response.role.into(),
            url_img: response.url_img,
            goal: response.goal.map(Into::into),
        }
    }

    pub fn to_auth_response_dto(&self, user: User) -> AuthResponseDTO {
        AuthResponseDTO {
            user: self.to_user_response_dto(user),
        }
    }

    pub fn to_login_request(&self, request: LoginRequestDTO) -> LoginRequest {
        LoginRequest {
            email: request.email,
            password: request.password,
        }
    }

    pub fn to_google_login_request(&self, request: GoogleLoginRequestDTO) -> GoogleLoginRequest {
        GoogleLoginRequest {
            id_token: request.id_token,
        }
    }

    pub fn to_verify_request(&self, request: VerifyRequestDTO) -> VerifyRequest {
        VerifyRequest {
            email: request.email,
            code: request.code as u32,
        }
    }

    pub fn to_update_user_request(&self, request: UpdateUserDTO) -> UpdateUserRequest {
        UpdateUserRequest {
            id: request.id,
            name: request.name,
            code: request.code as u32,
        }
    }

    pub fn to_update_email_request(&self, request: UpdateEmailDTO) -> EmailChangeRequest {
        EmailChangeRequest {
            id: request.id,
            email: request.email,
            code: request.code as u32,
        }
    }

    pub fn to_update_password_request(&self, request: UpdatePasswordDTO) -> PasswordChangeRequest {
        PasswordChangeRequest {
            id: request.id,
            password: request.password,
            code: request.code as u32,
        }
    }

    pub fn to_refresh_response() -> RefreshResponseDTO {
        RefreshResponseDTO {}
    }
}
