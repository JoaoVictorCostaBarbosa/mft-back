use crate::{
    application::app_state::app_state::AppState,
    interfaces::http::routers::{auth_router::auth_routers, user_router::user_routers},
};
use axum::Router;

pub mod auth_router;
pub mod user_router;

pub fn build_http() -> Router<AppState>
{
    Router::new()
        // Note que auth_routers e user_routers também devem retornar Router<AppState>
        .nest("/api", auth_routers()) // Assumindo que auth_routers também foi corrigido
        .nest("/api", user_routers())
}
