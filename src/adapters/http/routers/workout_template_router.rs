use crate::{
    adapters::http::handlers::workout_template::{
        add_exercise_to_wt::add_exercise_to_workout_template_handler,
        create_wt::create_workout_template_handler, delete_wt::delete_workout_template_handler,
        find_wt_by_id::find_workout_template_by_id_handler,
        read_user_wt::read_user_workout_templates_handler,
        remove_exercise_from_wt::remove_exercise_from_workout_template_handler,
        soft_delete_wt::soft_delete_workout_template_handler,
        update_wt_name::update_workout_template_name_handler,
    },
    application::app_state::app_state::AppState,
};
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
