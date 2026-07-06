mod adapters;
mod api_doc;
mod application;
mod db;
mod domain;
mod infrastructure;
#[cfg(test)]
mod test_support;
use crate::adapters::http::CookieConfig;
use crate::adapters::http::routers::build_http;
use crate::api_doc::ApiDoc;
use crate::application::app_state::AppState;
use crate::application::ports::CryptoService;
use crate::application::ports::JwtProvider;
use crate::infrastructure::config::LoadEnv;
use crate::infrastructure::providers::GoogleOAuthHttpProvider;
use crate::infrastructure::providers::R2Storage;
use crate::infrastructure::providers::mail::ResendEmailService;
use crate::infrastructure::repositories::postgres::RepositoryBundle;
use crate::infrastructure::security::Argon2Hasher;
use crate::infrastructure::security::HmacShaHasher;
use crate::infrastructure::security::jwt::JwtService;
use crate::infrastructure::system::{RandCodeGenerator, SystemClock, UuidTokenGenerator};
use axum::http::{HeaderValue, Method, header};
use axum::{Extension, Router};
use db::create_pool;
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

async fn root() -> &'static str {
    "Servidor está rodando"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
    let env = LoadEnv::new();
    tracing::info!(development_mode = env.app_development, "app starting");

    let pool = create_pool(&env.database_url).await;
    tracing::info!("sqlx pool: connection established");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let repos = RepositoryBundle::new(pool.clone());

    let crypto_service: Arc<dyn CryptoService> = Arc::new(Argon2Hasher {});

    let google_oauth_provider = Arc::new(GoogleOAuthHttpProvider::new(env.google_client_id));

    let jwt_service: Arc<dyn JwtProvider> =
        Arc::new(JwtService::new(env.secret_access_key, env.access_minutes));

    let hmac_sha_service = Arc::new(HmacShaHasher::new(env.secret_refresh_key));

    let r2_service = Arc::new(R2Storage::new(
        &env.r2_access_key_id,
        &env.r2_secret_access_key,
        &env.r2_bucket_name,
        &env.r2_public_base_url,
        &env.r2_s3_endpoint,
    ));

    let resend_service = Arc::new(ResendEmailService::new(
        env.smtp_pass,
        env.smtp_from.expect("SMTP_FROM is required"),
    ));

    let clock = Arc::new(SystemClock);
    let code_generator = Arc::new(RandCodeGenerator);
    let token_generator = Arc::new(UuidTokenGenerator);

    let app_state = AppState::new(
        repos.user_repo,
        repos.pending_user_repo,
        repos.refresh_token_repo,
        repos.pending_change_repo,
        repos.measurement_repo,
        repos.exercise_repo,
        repos.exercise_queries,
        repos.workout_plan_repo,
        repos.workout_session_repo,
        repos.workout_session_queries,
        repos.workout_template_repo,
        crypto_service,
        google_oauth_provider,
        hmac_sha_service,
        jwt_service,
        resend_service,
        r2_service,
        clock,
        code_generator,
        token_generator,
        env.refresh_days,
    );
    let cookie_config =
        CookieConfig::new(env.app_development, env.access_minutes, env.refresh_days);

    let cors_origins: Vec<HeaderValue> = env
        .cors_allowed_origins
        .iter()
        .map(|origin| {
            origin
                .parse::<HeaderValue>()
                .unwrap_or_else(|_| panic!("invalid CORS origin: {}", origin))
        })
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(cors_origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        .route("/", axum::routing::get(root))
        .merge(build_http())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(Extension(cookie_config))
        .layer(cors)
        .with_state(app_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], env.port));
    tracing::info!(%addr, "server running");

    tracing::info!(
        "API documentation in: http://localhost:{}/swagger-ui",
        env.port
    );

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
