use crate::adapters::http::handlers::exercise::create_exercise_handler;
use crate::adapters::http::handlers::exercise::delete_exercise_handler;
use crate::adapters::http::handlers::exercise::find_exercise_last_performances_handler;
use crate::adapters::http::handlers::exercise::get_exercise_by_id_handler;
use crate::adapters::http::handlers::exercise::read_exercises_handler;
use crate::adapters::http::handlers::exercise::read_personal_records_handler;
use crate::adapters::http::handlers::exercise::search_equipment_handler;
use crate::adapters::http::handlers::exercise::search_exercise_type_handler;
use crate::adapters::http::handlers::exercise::search_myscle_group_exercise;
use crate::adapters::http::handlers::exercise::soft_delete_exercise_handler;
use crate::adapters::http::handlers::exercise::update_exercise_handler;
use crate::application::app_state::AppState;
use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

pub fn exercise_routers() -> Router<AppState> {
    Router::new()
        .route("/exercises", post(create_exercise_handler))
        .route("/exercises", get(read_exercises_handler))
        .route(
            "/exercises/last-performances",
            post(find_exercise_last_performances_handler),
        )
        .route(
            "/exercises/personal-records",
            get(read_personal_records_handler),
        )
        .route("/exercises/:id", get(get_exercise_by_id_handler))
        .route(
            "/exercises/type/:exercise_type",
            get(search_exercise_type_handler),
        )
        .route(
            "/exercises/equipment/:equipment",
            get(search_equipment_handler),
        )
        .route(
            "/exercises/muscle-group/:muscle_group",
            get(search_myscle_group_exercise),
        )
        .route("/exercises", put(update_exercise_handler))
        .route(
            "/exercises/:id/soft-delete",
            patch(soft_delete_exercise_handler),
        )
        .route("/exercises/:id", delete(delete_exercise_handler))
}
