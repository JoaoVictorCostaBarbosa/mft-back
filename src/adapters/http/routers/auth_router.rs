use crate::adapters::http::handlers::auth::create_user_handler;
use crate::adapters::http::handlers::auth::google_login_handler;
use crate::adapters::http::handlers::auth::login_user_handler;
use crate::adapters::http::handlers::auth::logout_handler;
use crate::adapters::http::handlers::auth::refresh_access_handler;
use crate::adapters::http::handlers::auth::verify_user_handler;
use crate::application::app_state::AppState;
use axum::{
    Router,
    routing::{patch, post},
};

pub fn auth_routers() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(create_user_handler))
        .route("/auth/verify", post(verify_user_handler))
        .route("/auth/login", post(login_user_handler))
        .route("/auth/google", post(google_login_handler))
        .route("/auth/refresh", post(refresh_access_handler))
        .route("/auth/logout", patch(logout_handler))
}
