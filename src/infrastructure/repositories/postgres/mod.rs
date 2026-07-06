use crate::application::ports::ExerciseQueries;
use crate::application::ports::WorkoutSessionQueries;
use crate::domain::repositories::ExerciseRepository;
use crate::domain::repositories::MeasurementRepository;
use crate::domain::repositories::PendingChangesRepository;
use crate::domain::repositories::PendingUserRepository;
use crate::domain::repositories::RefreshTokenRepository;
use crate::domain::repositories::UserRepository;
use crate::domain::repositories::WorkoutPlanRepository;
use crate::domain::repositories::WorkoutSessionRepository;
use crate::domain::repositories::WorkoutTemplateRepository;
use sqlx::PgPool;
use std::sync::Arc;

mod error;
mod exercise_repository_sqlx;
mod measurement_repository_sqlx;
mod pending_change_repository_sqlx;
mod pending_user_repository_sqlx;
mod refresh_token_repository_sqlx;
mod user_repository_sqlx;
mod workout_plan_repository_sqlx;
mod workout_session_repository_sqlx;
mod workout_template_repository_sqlx;

pub use exercise_repository_sqlx::ExerciseRepositorySqlx;
pub use measurement_repository_sqlx::MeasurementRepositorySqlx;
pub use pending_change_repository_sqlx::PendingChangeRepositorySqlx;
pub use pending_user_repository_sqlx::PendingUserRepositorySqlx;
pub use refresh_token_repository_sqlx::RefreshTokenRepositorySqlx;
pub use user_repository_sqlx::UserRepositorySQLx;
pub use workout_plan_repository_sqlx::WorkoutPlanRepositorySQLx;
pub use workout_session_repository_sqlx::WorkoutSessionRepositorySqlx;
pub use workout_template_repository_sqlx::WorkoutTemplateRepositorySQLX;

pub struct RepositoryBundle {
    pub user_repo: Arc<dyn UserRepository>,
    pub pending_user_repo: Arc<dyn PendingUserRepository>,
    pub refresh_token_repo: Arc<dyn RefreshTokenRepository>,
    pub pending_change_repo: Arc<dyn PendingChangesRepository>,
    pub measurement_repo: Arc<dyn MeasurementRepository>,
    pub exercise_repo: Arc<dyn ExerciseRepository>,
    pub exercise_queries: Arc<dyn ExerciseQueries>,
    pub workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
    pub workout_session_repo: Arc<dyn WorkoutSessionRepository>,
    pub workout_session_queries: Arc<dyn WorkoutSessionQueries>,
    pub workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
}

impl RepositoryBundle {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_repo: Arc::new(UserRepositorySQLx::new(pool.clone())),
            pending_user_repo: Arc::new(PendingUserRepositorySqlx::new(pool.clone())),
            refresh_token_repo: Arc::new(RefreshTokenRepositorySqlx::new(pool.clone())),
            pending_change_repo: Arc::new(PendingChangeRepositorySqlx::new(pool.clone())),
            measurement_repo: Arc::new(MeasurementRepositorySqlx::new(pool.clone())),
            exercise_repo: Arc::new(ExerciseRepositorySqlx::new(pool.clone())),
            exercise_queries: Arc::new(ExerciseRepositorySqlx::new(pool.clone())),
            workout_plan_repo: Arc::new(WorkoutPlanRepositorySQLx::new(pool.clone())),
            workout_session_repo: Arc::new(WorkoutSessionRepositorySqlx::new(pool.clone())),
            workout_session_queries: Arc::new(WorkoutSessionRepositorySqlx::new(pool.clone())),
            workout_template_repo: Arc::new(WorkoutTemplateRepositorySQLX::new(pool.clone())),
        }
    }
}
