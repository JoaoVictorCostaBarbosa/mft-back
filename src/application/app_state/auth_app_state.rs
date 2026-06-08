use crate::{
    application::{
        config::auth_config::AuthConfig,
        interfaces::pending_user_repository::PendingUserRepository,
        services::auth_service::create_refresh_token::IssueRefreshToken,
        usecase::auth::{
            create_user::CreateUser, login_user::LoginUser, login_with_google::LoginWithGoogle,
            logout::Logout, refresh_session::RefreshSession, verify_user::VerifyUser,
        },
    },
    domain::{
        repositories::{
            refresh_token_repository::RefreshTokenRepository, user_repository::UserRepository,
        },
        services::{
            cripto::CriptoService, google_oauth::GoogleOAuthProvider, jwt::JwtProvider,
            refresh_token_hasher::RefreshTokenHasher, smtp::SmtpService,
        },
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthAppState {
    pub create_user: Arc<CreateUser>,
    pub verify_user: Arc<VerifyUser>,
    pub login_user: Arc<LoginUser>,
    pub login_with_google: Arc<LoginWithGoogle>,
    pub refresh_session: Arc<RefreshSession>,
    pub logout: Arc<Logout>,
    pub issue_token_service: Arc<IssueRefreshToken>,
    pub user_repo: Arc<dyn UserRepository>,
}

impl AuthAppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_user_repo: Arc<dyn PendingUserRepository>,
        refresh_repo: Arc<dyn RefreshTokenRepository>,
        cripto_service: Arc<dyn CriptoService>,
        google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
        jwt_service: Arc<dyn JwtProvider>,
        hash_service: Arc<dyn RefreshTokenHasher>,
        smtp_service: Arc<dyn SmtpService>,
        auth_config: Arc<AuthConfig>,
    ) -> Self {
        Self {
            create_user: Arc::new(CreateUser::new(
                user_repo.clone(),
                pending_user_repo.clone(),
                cripto_service.clone(),
                smtp_service.clone(),
            )),
            verify_user: Arc::new(VerifyUser::new(
                user_repo.clone(),
                pending_user_repo.clone(),
            )),
            login_user: Arc::new(LoginUser::new(user_repo.clone(), cripto_service.clone())),
            login_with_google: Arc::new(LoginWithGoogle::new(
                user_repo.clone(),
                cripto_service.clone(),
                google_oauth_provider,
            )),
            refresh_session: Arc::new(RefreshSession::new(
                refresh_repo.clone(),
                user_repo.clone(),
                jwt_service.clone(),
                hash_service.clone(),
                auth_config.refresh_token_exp_time,
            )),
            logout: Arc::new(Logout::new(refresh_repo.clone(), hash_service.clone())),
            issue_token_service: Arc::new(IssueRefreshToken::new(
                refresh_repo.clone(),
                hash_service.clone(),
                auth_config.refresh_token_exp_time,
            )),
            user_repo,
        }
    }
}
