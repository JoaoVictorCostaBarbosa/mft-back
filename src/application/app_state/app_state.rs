use crate::application::app_state::AuthAppState;
use crate::application::app_state::ExerciseAppState;
use crate::application::app_state::MeasurementAppState;
use crate::application::app_state::UserAppState;
use crate::application::app_state::WorkoutPlanAppState;
use crate::application::app_state::WorkoutSessionAppState;
use crate::application::app_state::WorkoutTemplateAppState;
use crate::application::config::AuthConfig;
use crate::application::ports::Clock;
use crate::application::ports::CodeGenerator;
use crate::application::ports::CryptoService;
use crate::application::ports::ExerciseQueries;
use crate::application::ports::FileStorage;
use crate::application::ports::GoogleOAuthProvider;
use crate::application::ports::JwtProvider;
use crate::application::ports::Mailer;
use crate::application::ports::RefreshTokenHasher;
use crate::application::ports::TokenGenerator;
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
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub auth: AuthAppState,
    pub user: UserAppState,
    pub measurement: MeasurementAppState,
    pub exercise: ExerciseAppState,
    pub workout_plan: WorkoutPlanAppState,
    pub workout_session: WorkoutSessionAppState,
    pub workout_template: WorkoutTemplateAppState,
    pub jwt_service: Arc<dyn JwtProvider>,
}

impl AppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_user_repo: Arc<dyn PendingUserRepository>,
        refresh_token_repo: Arc<dyn RefreshTokenRepository>,
        pending_change_repo: Arc<dyn PendingChangesRepository>,
        measurement_repo: Arc<dyn MeasurementRepository>,
        exercise_repo: Arc<dyn ExerciseRepository>,
        exercise_queries: Arc<dyn ExerciseQueries>,
        workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
        workout_session_repo: Arc<dyn WorkoutSessionRepository>,
        workout_session_queries: Arc<dyn WorkoutSessionQueries>,
        workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
        crypto_service: Arc<dyn CryptoService>,
        google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
        hash_service: Arc<dyn RefreshTokenHasher>,
        jwt_service: Arc<dyn JwtProvider>,
        mailer: Arc<dyn Mailer>,
        file_storage: Arc<dyn FileStorage>,
        clock: Arc<dyn Clock>,
        code_generator: Arc<dyn CodeGenerator>,
        token_generator: Arc<dyn TokenGenerator>,
        refresh_exp_days: i64,
    ) -> Self {
        Self {
            auth: AuthAppState::new(
                user_repo.clone(),
                pending_user_repo.clone(),
                refresh_token_repo.clone(),
                crypto_service.clone(),
                google_oauth_provider,
                jwt_service.clone(),
                hash_service.clone(),
                mailer.clone(),
                clock.clone(),
                code_generator.clone(),
                token_generator.clone(),
                Arc::new(AuthConfig::new(refresh_exp_days)),
            ),
            user: UserAppState::new(
                user_repo.clone(),
                pending_change_repo.clone(),
                crypto_service.clone(),
                mailer.clone(),
                file_storage.clone(),
                clock.clone(),
                code_generator.clone(),
            ),
            measurement: MeasurementAppState::new(measurement_repo.clone()),
            exercise: ExerciseAppState::new(exercise_repo.clone(), exercise_queries),
            workout_plan: WorkoutPlanAppState::new(
                workout_plan_repo.clone(),
                workout_template_repo.clone(),
            ),
            workout_session: WorkoutSessionAppState::new(
                workout_session_repo.clone(),
                workout_session_queries,
                workout_plan_repo.clone(),
                workout_template_repo.clone(),
            ),
            workout_template: WorkoutTemplateAppState::new(
                workout_template_repo.clone(),
                exercise_repo.clone(),
            ),
            jwt_service,
        }
    }
}
