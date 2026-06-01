use crate::adapters::http::dtos::user_dto::*;
use crate::application::dtos::auth::login_request::LoginRequest;
use crate::application::dtos::auth::refresh_response::RefreshResponse;
use crate::application::dtos::auth::user_create::UserCreate;
use crate::application::dtos::auth::verify_request::VerifyRequest;
use crate::application::dtos::user::email_change_request::EmailChangeRequest;
use crate::application::dtos::user::password_change_request::PasswordChangeRequest;
use crate::application::dtos::user::update_user_request::UpdateUserRequest;
use crate::domain::entities::user::User;

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
        }
    }

    pub fn to_auth_response_dto(
        &self,
        user: User,
        access: String,
        refresh: String,
    ) -> AuthResponseDTO {
        AuthResponseDTO {
            user: self.to_user_response_dto(user),
            access: access,
            refresh: refresh,
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

    pub fn to_refresh_response(token: RefreshResponse) -> RefreshResponseDTO {
        RefreshResponseDTO {
            access: token.access,
            refresh: token.refresh,
        }
    }
}
