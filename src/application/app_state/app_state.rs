use crate::{
    application::{
        app_state::{
            auth_app_state::AuthAppState, exercise_app_state::ExerciseAppState,
            measurement_app_state::MeasurementAppState, user_app_state::UserAppState,
            workout_plan_app_state::WorkoutPlanAppState,
            workout_session_app_state::WorkoutSessionAppState,
            workout_template_app_state::WorkoutTemplateAppState,
        },
        config::auth_config::AuthConfig,
        interfaces::{
            pending_change_repository::PendingChangesRepository,
            pending_user_repository::PendingUserRepository,
        },
    },
    domain::{
        repositories::{
            exercise_repository::ExerciseRepository, measurement_repository::MeasurementRepository,
            refresh_token_repository::RefreshTokenRepository, user_repository::UserRepository,
            workout_plan_repository::WorkoutPlanRepository,
            workout_session_repository::WorkoutSessionRepository,
            workout_template_repository::WorkoutTemplateRepository,
        },
        services::{
            bucket_storage::BucketStorage, cripto::CriptoService,
            google_oauth::GoogleOAuthProvider, jwt::JwtProvider,
            refresh_token_hasher::RefreshTokenHasher, smtp::SmtpService,
        },
    },
};
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
        workout_plan_repo: Arc<dyn WorkoutPlanRepository>,
        workout_session_repo: Arc<dyn WorkoutSessionRepository>,
        workout_template_repo: Arc<dyn WorkoutTemplateRepository>,
        cripto_service: Arc<dyn CriptoService>,
        google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
        hash_service: Arc<dyn RefreshTokenHasher>,
        jwt_service: Arc<dyn JwtProvider>,
        smtp_service: Arc<dyn SmtpService>,
        bucket_service: Arc<dyn BucketStorage>,
        refresh_exp_days: i64,
    ) -> Self {
        Self {
            auth: AuthAppState::new(
                user_repo.clone(),
                pending_user_repo.clone(),
                refresh_token_repo.clone(),
                cripto_service.clone(),
                google_oauth_provider,
                jwt_service.clone(),
                hash_service.clone(),
                smtp_service.clone(),
                Arc::new(AuthConfig::new(refresh_exp_days)),
            ),
            user: UserAppState::new(
                user_repo.clone(),
                pending_change_repo.clone(),
                cripto_service.clone(),
                smtp_service.clone(),
                bucket_service.clone(),
            ),
            measurement: MeasurementAppState::new(measurement_repo.clone()),
            exercise: ExerciseAppState::new(exercise_repo.clone()),
            workout_plan: WorkoutPlanAppState::new(
                workout_plan_repo.clone(),
                workout_template_repo.clone(),
            ),
            workout_session: WorkoutSessionAppState::new(
                workout_session_repo.clone(),
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
