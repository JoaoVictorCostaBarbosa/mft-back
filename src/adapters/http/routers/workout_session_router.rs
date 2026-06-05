use crate::{
    adapters::http::handlers::workout_session::{
        add_exercise_to_session::add_exercise_to_workout_session_handler,
        add_set_to_session::add_set_to_workout_session_handler,
        find_current_session::find_current_workout_session_handler,
        finish_session::finish_workout_session_handler,
        read_session_history::read_workout_session_history_handler,
        read_weekly_summary::read_workout_session_weekly_summary_handler,
        start_session::start_workout_session_handler,
    },
    application::app_state::app_state::AppState,
};
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
            "/workout-sessions/:session_id/exercises",
            post(add_exercise_to_workout_session_handler),
        )
        .route(
            "/workout-sessions/:session_id/sets",
            post(add_set_to_workout_session_handler),
        )
}
