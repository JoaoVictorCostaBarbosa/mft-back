use crate::{
    application::app_state::app_state::AppState,
    interfaces::http::handlers::user::{
        delete_user::delete_user_handler, find_users::find_users_handler,
        restore_user::restore_user_handler, send_code::send_code_handler,
        soft_delete_user::soft_delete_user_handler, update_avatar::update_avatar_handler,
        update_email::update_email_handler, update_password::update_password_handler,
        update_user::update_user_handler,
    },
};
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub fn user_routers() -> Router<AppState> {
    Router::new()
        .route("/users", get(find_users_handler))
        .route("/users/send-code", post(send_code_handler))
        .route("/users", patch(update_user_handler))
        .route("/users/me/email", patch(update_email_handler))
        .route("/users/me/password", patch(update_password_handler))
        .route("/users/me/avatar", patch(update_avatar_handler))
        .route(
            "/users/:user_id/soft-delete",
            patch(soft_delete_user_handler),
        )
        .route("/users/:user_id/restore", patch(restore_user_handler))
        .route("/users/:id", delete(delete_user_handler))
}
