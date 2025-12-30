use crate::adapters::http::dtos::user_dto::CreateUserRequestDTO;
use utoipa::{
    OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Caminho dos handlers
        crate::adapters::http::handlers::auth::create_user::create_user_handler,
    ),
    components(
        schemas(
            // dtos que derivam ToSchema
            CreateUserRequestDTO
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
