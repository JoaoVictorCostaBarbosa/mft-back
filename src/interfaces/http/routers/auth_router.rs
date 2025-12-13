use crate::{
    application::app_state::app_state::AppState,
    interfaces::http::handlers::user::{
        create_user::create_user_handler, login_user::login_user_handler,
        verify_user::verify_user_handler,
    },
};
use axum::{Router, extract::FromRef, routing::post};

pub fn auth_routers<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    AppState: FromRef<S>,
{
    Router::new()
        .route("/auth/register", post(create_user_handler))
        .route("/auth/verify", post(verify_user_handler))
        .route("/auth/login", post(login_user_handler))
}
