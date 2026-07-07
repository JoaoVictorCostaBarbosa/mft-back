use crate::adapters::http::handlers::user::delete_user_handler;
use crate::adapters::http::handlers::user::find_users_handler;
use crate::adapters::http::handlers::user::get_current_user_handler;
use crate::adapters::http::handlers::user::restore_user_handler;
use crate::adapters::http::handlers::user::send_code_handler;
use crate::adapters::http::handlers::user::soft_delete_user_handler;
use crate::adapters::http::handlers::user::update_avatar_handler;
use crate::adapters::http::handlers::user::update_email_handler;
use crate::adapters::http::handlers::user::update_goal_handler;
use crate::adapters::http::handlers::user::update_password_handler;
use crate::adapters::http::handlers::user::update_user_handler;
use crate::adapters::http::extractors::MAX_IMAGE_SIZE_BYTES;
use crate::application::app_state::AppState;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{delete, get, patch, post},
};

pub fn user_routers() -> Router<AppState> {
    Router::new()
        .route("/users", get(find_users_handler))
        .route("/users/me", get(get_current_user_handler))
        .route("/users/send-code", post(send_code_handler))
        .route("/users", patch(update_user_handler))
        .route("/users/me/email", patch(update_email_handler))
        .route("/users/me/password", patch(update_password_handler))
        .route(
            "/users/me/avatar",
            // Folga sobre o limite do arquivo para o overhead do multipart.
            patch(update_avatar_handler).layer(DefaultBodyLimit::max(MAX_IMAGE_SIZE_BYTES + 1024 * 1024)),
        )
        .route("/users/me/goal", patch(update_goal_handler))
        .route(
            "/users/:user_id/soft-delete",
            patch(soft_delete_user_handler),
        )
        .route("/users/:user_id/restore", patch(restore_user_handler))
        .route("/users/:id", delete(delete_user_handler))
}
