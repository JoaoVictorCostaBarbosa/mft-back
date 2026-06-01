use crate::{
    adapters::http::handlers::auth::{
        create_user::create_user_handler, login_user::login_user_handler, logout::logout_handler,
        refresh_token::refresh_access_handler, verify_user::verify_user_handler,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Router,
    routing::{patch, post},
};

pub fn auth_routers() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(create_user_handler))
        .route("/auth/verify", post(verify_user_handler))
        .route("/auth/login", post(login_user_handler))
        .route("/auth/refresh", post(refresh_access_handler))
        .route("/auth/logout", patch(logout_handler))
}
