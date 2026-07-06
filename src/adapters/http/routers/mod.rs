use crate::application::app_state::AppState;
use axum::Router;

mod auth_router;
mod exercise_router;
mod measurement_router;
mod user_router;
mod workout_plan_router;
mod workout_session_router;
mod workout_template_router;

pub use auth_router::auth_routers;
pub use exercise_router::exercise_routers;
pub use measurement_router::measurement_routers;
pub use user_router::user_routers;
pub use workout_plan_router::workout_plan_routers;
pub use workout_session_router::workout_session_routers;
pub use workout_template_router::workout_template_routers;

pub fn build_http() -> Router<AppState> {
    Router::new().nest(
        "/api",
        Router::new()
            .merge(auth_routers())
            .merge(user_routers())
            .merge(measurement_routers())
            .merge(exercise_routers())
            .merge(workout_plan_routers())
            .merge(workout_session_routers())
            .merge(workout_template_routers()),
    )
}
