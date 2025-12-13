use crate::application::dtos::user::auth_reponse::AuthResponse;
use crate::application::dtos::user::email_change_request::EmailChangeRequest;
use crate::application::dtos::user::login_request::LoginRequest;
use crate::application::dtos::user::password_change_request::PasswordChangeRequest;
use crate::application::dtos::user::update_user_request::UpdateUserRequest;
use crate::application::dtos::user::user_create::UserCreate;
use crate::application::dtos::user::user_response::UserResponse;
use crate::application::dtos::user::verify_request::VerifyRequest;
use crate::interfaces::http::dtos::user_dto::{
    AuthResponseDTO, CreateUserRequestDTO, LoginRequestDTO, UpdateEmailDTO, UpdatePasswordDTO,
    UpdateUserDTO, UserResponseDTO, VerifyRequestDTO,
};

pub struct UserMappers;

impl UserMappers {
    pub fn to_user_create_dto(&self, request: CreateUserRequestDTO) -> UserCreate {
        UserCreate {
            name: request.name,
            email: request.email,
            password: request.password,
        }
    }

    pub fn to_user_response_dto(&self, response: UserResponse) -> UserResponseDTO {
        UserResponseDTO {
            id: response.id,
            name: response.name.clone(),
            email: response.email.clone(),
            url_img: response.url_img,
        }
    }

    pub fn to_auth_response_dto(&self, response: AuthResponse) -> AuthResponseDTO {
        AuthResponseDTO {
            user: self.to_user_response_dto(response.user),
            access: response.access,
            refresh: response.refresh,
        }
    }

    pub fn to_login_request(&self, request: LoginRequestDTO) -> LoginRequest {
        LoginRequest {
            email: request.email,
            password: request.password,
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
}
