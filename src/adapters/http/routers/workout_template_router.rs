use crate::adapters::http::handlers::workout_template::add_exercise_to_workout_template_handler;
use crate::adapters::http::handlers::workout_template::create_workout_template_handler;
use crate::adapters::http::handlers::workout_template::delete_workout_template_handler;
use crate::adapters::http::handlers::workout_template::find_workout_template_by_id_handler;
use crate::adapters::http::handlers::workout_template::read_user_workout_templates_handler;
use crate::adapters::http::handlers::workout_template::remove_exercise_from_workout_template_handler;
use crate::adapters::http::handlers::workout_template::soft_delete_workout_template_handler;
use crate::adapters::http::handlers::workout_template::update_workout_template_name_handler;
use crate::application::app_state::AppState;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub fn workout_template_routers() -> Router<AppState> {
    Router::new()
        .route("/workout-templates", post(create_workout_template_handler))
        .route(
            "/workout-templates",
            get(read_user_workout_templates_handler),
        )
        .route(
            "/workout-templates/:workout_id",
            get(find_workout_template_by_id_handler),
        )
        .route(
            "/workout-templates/change-name",
            patch(update_workout_template_name_handler),
        )
        .route(
            "/workout-templates/:workout_id/soft-delete",
            delete(soft_delete_workout_template_handler),
        )
        .route(
            "/workout-templates/:workout_id",
            delete(delete_workout_template_handler),
        )
        .route(
            "/workout-templates/add-exercise",
            post(add_exercise_to_workout_template_handler),
        )
        .route(
            "/workout-templates/remove-exercise",
            post(remove_exercise_from_workout_template_handler),
        )
}
