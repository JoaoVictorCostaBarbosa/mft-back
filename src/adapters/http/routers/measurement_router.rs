use crate::adapters::http::handlers::measurement::create_measurement_handler;
use crate::adapters::http::handlers::measurement::delete_measurement_handler;
use crate::adapters::http::handlers::measurement::find_measurement_by_id_handler;
use crate::adapters::http::handlers::measurement::find_measurements_handler;
use crate::adapters::http::handlers::measurement::soft_delete_measurement_handler;
use crate::application::app_state::AppState;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub fn measurement_routers() -> Router<AppState> {
    Router::new()
        .route("/measurements", post(create_measurement_handler))
        .route("/measurements", get(find_measurements_handler))
        .route("/measurements/:id", get(find_measurement_by_id_handler))
        .route("/measurements/:id", patch(soft_delete_measurement_handler))
        .route("/measurements/:id", delete(delete_measurement_handler))
}
