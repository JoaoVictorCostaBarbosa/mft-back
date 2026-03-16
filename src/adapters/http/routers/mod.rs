use crate::{
    adapters::http::routers::{
        auth_router::auth_routers, exercise_router::exercise_routers,
        measurement_router::measurement_routers, user_router::user_routers,
        workout_plan_router::workout_plan_routers,
        workout_template_router::workout_template_routers,
    },
    application::app_state::app_state::AppState,
};
use axum::Router;

pub mod auth_router;
pub mod exercise_router;
pub mod measurement_router;
pub mod user_router;
pub mod workout_plan_router;
pub mod workout_template_router;

pub fn build_http() -> Router<AppState> {
    Router::new().nest(
        "/api",
        Router::new()
            .merge(auth_routers())
            .merge(user_routers())
            .merge(measurement_routers())
            .merge(exercise_routers())
            .merge(workout_plan_routers())
            .merge(workout_template_routers()),
    )
}
