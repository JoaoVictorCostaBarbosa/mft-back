use crate::adapters::http::dtos::user_dto::{
    AuthResponseDTO, CreateUserRequestDTO, LoginRequestDTO, UpdateUserDTO, UserResponseDTO,
    VerifyRequestDTO,
};
use utoipa::{
    OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::adapters::http::handlers::user::create_user::create_user_handler,
        crate::adapters::http::handlers::user::login_user::login_user_handler,
        crate::adapters::http::handlers::user::verify_user::verify_user_handler,
        crate::adapters::http::handlers::user::soft_delete_user::soft_delete_user_handler,
        crate::adapters::http::handlers::user::update_user::update_user_handler,
        crate::adapters::http::handlers::user::send_code::send_code_handler,
    ),
    components(
        schemas(
            CreateUserRequestDTO,
            LoginRequestDTO,
            VerifyRequestDTO,
            UserResponseDTO,
            AuthResponseDTO,
            UpdateUserDTO,
        )
    ),
    tags(
        (name = "Auth", description = "User authentication"),
        (name = "Users", description = "User management"),
    ),
    modifiers(&SecurityAddon),
    info(title = "myFitTracker-API", version = "0.1.0")
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
