mod adapters;
mod api_doc;
mod application;
mod db;
mod domain;
mod infrastructure;
use crate::{
    adapters::http::routers::build_http,
    api_doc::ApiDoc,
    application::app_state::app_state::AppState,
    domain::services::{cripto::CriptoService, jwt::JwtProvider},
    infrastructure::{
        config::env::LoadEnv,
        providers::{mail::lettre_sending::LettreSmtpService, r2_storage::R2Storage},
        repositories::postgres::RepositoryBundle,
        security::{
            argon2_hasher::Argon2Hasher, hmac_sha_hasher::HmacShaHasher,
            jwt::jwt_token_service::JwtService,
        },
    },
};
use axum::Router;
use axum::http::{HeaderValue, Method, header};
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
    let env = LoadEnv::new();
    println!("INFO app: development mode = {}", env.app_development);

    let pool = create_pool(&env.database_url).await;
    println!("INFO sqlx::pool: connection established");

    sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("Failed to run migrations");

    let repos = RepositoryBundle::new(pool.clone());

    let cripto_service: Arc<dyn CriptoService> = Arc::new(Argon2Hasher {});

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

    let lettre_service = Arc::new(LettreSmtpService::new(
        env.smtp_host,
        env.smtp_port,
        env.smtp_secure,
        env.smtp_user,
        env.smtp_pass,
        env.smtp_from,
    ));

    let app_state = AppState::new(
        repos.user_repo,
        repos.pending_user_repo,
        repos.refresh_token_repo,
        repos.pending_change_repo,
        repos.measurement_repo,
        repos.exercise_repo,
        repos.workout_plan_repo,
        repos.workout_template_repo,
        cripto_service,
        hmac_sha_service,
        jwt_service,
        lettre_service,
        r2_service,
        env.refresh_days,
    );

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
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", axum::routing::get(root))
        .merge(build_http())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state(app_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], env.port));
    println!("INFO server: running on {}", addr);

    println!("API documentation in: http://localhost:{}/swagger-ui", env.port);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
