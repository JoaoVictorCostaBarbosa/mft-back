use crate::application::config::AuthConfig;
use crate::application::ports::Clock;
use crate::application::ports::CodeGenerator;
use crate::application::ports::CryptoService;
use crate::application::ports::GoogleOAuthProvider;
use crate::application::ports::JwtProvider;
use crate::application::ports::Mailer;
use crate::application::ports::RefreshTokenHasher;
use crate::application::ports::TokenGenerator;
use crate::application::usecase::auth::CreateUser;
use crate::application::usecase::auth::GetAuthenticatedUser;
use crate::application::usecase::auth::IssueRefreshToken;
use crate::application::usecase::auth::LoginUser;
use crate::application::usecase::auth::LoginWithGoogle;
use crate::application::usecase::auth::Logout;
use crate::application::usecase::auth::RefreshSession;
use crate::application::usecase::auth::VerifyUser;
use crate::domain::repositories::PendingUserRepository;
use crate::domain::repositories::RefreshTokenRepository;
use crate::domain::repositories::UserRepository;
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
    pub get_authenticated_user: Arc<GetAuthenticatedUser>,
}

impl AuthAppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pending_user_repo: Arc<dyn PendingUserRepository>,
        refresh_repo: Arc<dyn RefreshTokenRepository>,
        crypto_service: Arc<dyn CryptoService>,
        google_oauth_provider: Arc<dyn GoogleOAuthProvider>,
        jwt_service: Arc<dyn JwtProvider>,
        hash_service: Arc<dyn RefreshTokenHasher>,
        mailer: Arc<dyn Mailer>,
        clock: Arc<dyn Clock>,
        code_generator: Arc<dyn CodeGenerator>,
        token_generator: Arc<dyn TokenGenerator>,
        auth_config: Arc<AuthConfig>,
    ) -> Self {
        Self {
            create_user: Arc::new(CreateUser::new(
                user_repo.clone(),
                pending_user_repo.clone(),
                crypto_service.clone(),
                mailer.clone(),
                clock.clone(),
                code_generator.clone(),
            )),
            verify_user: Arc::new(VerifyUser::new(
                user_repo.clone(),
                pending_user_repo.clone(),
            )),
            login_user: Arc::new(LoginUser::new(user_repo.clone(), crypto_service.clone())),
            login_with_google: Arc::new(LoginWithGoogle::new(
                user_repo.clone(),
                crypto_service.clone(),
                google_oauth_provider,
                token_generator.clone(),
            )),
            refresh_session: Arc::new(RefreshSession::new(
                refresh_repo.clone(),
                user_repo.clone(),
                jwt_service.clone(),
                hash_service.clone(),
                token_generator.clone(),
                clock.clone(),
                auth_config.refresh_token_exp_time,
            )),
            logout: Arc::new(Logout::new(refresh_repo.clone(), hash_service.clone())),
            issue_token_service: Arc::new(IssueRefreshToken::new(
                refresh_repo.clone(),
                hash_service.clone(),
                token_generator.clone(),
                clock.clone(),
                auth_config.refresh_token_exp_time,
            )),
            get_authenticated_user: Arc::new(GetAuthenticatedUser::new(user_repo)),
        }
    }
}
