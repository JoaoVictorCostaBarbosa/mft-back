use crate::adapters::http::handlers::workout_session::add_exercise_to_workout_session_handler;
use crate::adapters::http::handlers::workout_session::add_set_to_workout_session_handler;
use crate::adapters::http::handlers::workout_session::cancel_workout_session_handler;
use crate::adapters::http::handlers::workout_session::delete_workout_session_set_handler;
use crate::adapters::http::handlers::workout_session::find_current_workout_session_handler;
use crate::adapters::http::handlers::workout_session::finish_workout_session_handler;
use crate::adapters::http::handlers::workout_session::read_workout_session_history_handler;
use crate::adapters::http::handlers::workout_session::read_workout_session_weekly_summary_handler;
use crate::adapters::http::handlers::workout_session::remove_exercise_from_workout_session_handler;
use crate::adapters::http::handlers::workout_session::reorder_workout_session_exercises_handler;
use crate::adapters::http::handlers::workout_session::start_workout_session_handler;
use crate::adapters::http::handlers::workout_session::update_workout_session_set_handler;
use crate::application::app_state::AppState;
use axum::{
    Router,
    routing::{get, patch, post},
};

pub fn workout_session_routers() -> Router<AppState> {
    Router::new()
        .route("/workout-sessions", post(start_workout_session_handler))
        .route(
            "/workout-sessions/current",
            get(find_current_workout_session_handler),
        )
        .route(
            "/workout-sessions/history",
            get(read_workout_session_history_handler),
        )
        .route(
            "/workout-sessions/weekly-summary",
            get(read_workout_session_weekly_summary_handler),
        )
        .route(
            "/workout-sessions/:session_id/finish",
            patch(finish_workout_session_handler),
        )
        .route(
            "/workout-sessions/:session_id",
            axum::routing::delete(cancel_workout_session_handler),
        )
        .route(
            "/workout-sessions/:session_id/exercises",
            post(add_exercise_to_workout_session_handler),
        )
        .route(
            "/workout-sessions/:session_id/exercises/reorder",
            patch(reorder_workout_session_exercises_handler),
        )
        .route(
            "/workout-sessions/:session_id/exercises/:session_exercise_id",
            axum::routing::delete(remove_exercise_from_workout_session_handler),
        )
        .route(
            "/workout-sessions/:session_id/sets",
            post(add_set_to_workout_session_handler),
        )
        .route(
            "/workout-sessions/:session_id/sets/:set_id",
            patch(update_workout_session_set_handler).delete(delete_workout_session_set_handler),
        )
}
