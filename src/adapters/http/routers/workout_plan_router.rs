use crate::adapters::http::handlers::workout_plan::add_routine_item_to_workout_plan_handler;
use crate::adapters::http::handlers::workout_plan::add_workout_template_to_workout_plan_handler;
use crate::adapters::http::handlers::workout_plan::create_workout_plan_handler;
use crate::adapters::http::handlers::workout_plan::delete_workout_plan_handler;
use crate::adapters::http::handlers::workout_plan::find_next_routine_item_handler;
use crate::adapters::http::handlers::workout_plan::find_user_current_workout_plan;
use crate::adapters::http::handlers::workout_plan::find_workout_plan_by_id_handler;
use crate::adapters::http::handlers::workout_plan::read_user_workout_plans_summary_handler;
use crate::adapters::http::handlers::workout_plan::remove_routine_item_handler;
use crate::adapters::http::handlers::workout_plan::remove_workout_template_from_workout_plan_handler;
use crate::adapters::http::handlers::workout_plan::set_current_workout_plan;
use crate::adapters::http::handlers::workout_plan::soft_delete_workout_plan_handler;
use crate::adapters::http::handlers::workout_plan::update_routine_item_handler;
use crate::adapters::http::handlers::workout_plan::update_workout_plan_name_handler;
use crate::application::app_state::AppState;
use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

pub fn workout_plan_routers() -> Router<AppState> {
    Router::new()
        .route("/workout-plans", post(create_workout_plan_handler))
        .route(
            "/workout-plans",
            get(read_user_workout_plans_summary_handler),
        )
        .route(
            "/workout-plans/current",
            get(find_user_current_workout_plan),
        )
        .route(
            "/workout-plans/:workout_plan_id/current",
            put(set_current_workout_plan),
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
            "/workout-plans/:workout_plan_id/routine-items",
            post(add_routine_item_to_workout_plan_handler),
        )
        .route(
            "/workout-plans/:workout_plan_id/routine-items/:routine_item_id",
            patch(update_routine_item_handler).delete(remove_routine_item_handler),
        )
        .route(
            "/workout-plans/:workout_plan_id/next-routine-item",
            get(find_next_routine_item_handler),
        )
        .route(
            "/workout-plans/:workout_plan_id/workout-template/:workout_template_id",
            delete(remove_workout_template_from_workout_plan_handler),
        )
}
