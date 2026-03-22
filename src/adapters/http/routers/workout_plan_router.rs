use crate::{
    adapters::http::handlers::workout_plan::{
        add_workout_template_to_wp::add_workout_template_to_workout_plan_handler,
        create_wp::create_workout_plan_handler, delete_wp::delete_workout_plan_handler,
        find_wp_by_id::find_workout_plan_by_id_handler,
        read_user_wp_summary::read_user_workout_plans_summary_handler,
        remove_workout_template_from_wp::remove_workout_template_from_workout_plan_handler,
        soft_delete_wp::soft_delete_workout_plan_handler,
        update_wp_name::update_workout_plan_name_handler,
    },
    application::app_state::app_state::AppState,
};
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub fn workout_plan_routers() -> Router<AppState> {
    Router::new()
        .route("/workout-plans", post(create_workout_plan_handler))
        .route(
            "/workout-plans",
            get(read_user_workout_plans_summary_handler),
        )
        .route(
            "/workout-plans/:workout_plan_id",
            get(find_workout_plan_by_id_handler),
        )
        .route(
            "/workout-plans/change-name",
            patch(update_workout_plan_name_handler),
        )
        .route(
            "/workout-plans/:workout_plan_id",
            delete(delete_workout_plan_handler),
        )
        .route(
            "/workout-plans/:workout_plan_id/soft-delete",
            delete(soft_delete_workout_plan_handler),
        )
        .route(
            "/workout-plans/:workout_plan_id/workout-template/:workout_template_id",
            post(add_workout_template_to_workout_plan_handler),
        )
        .route(
            "/workout-plans/:workout_plan_id/workout-template/:workout_template_id",
            delete(remove_workout_template_from_workout_plan_handler),
        )
}
