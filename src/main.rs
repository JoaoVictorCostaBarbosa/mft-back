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
use db::create_pool;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

async fn root() -> &'static str {
    "Servidor está rodando"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let env = LoadEnv::new();

    let pool = create_pool(&env.database_url).await;
    println!("INFO sqlx::pool: connection established");

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

    let lettre_service = Arc::new(
        LettreSmtpService::new(
            env.smtp_host,
            env.smtp_port,
            env.smtp_secure,
            env.smtp_user,
            env.smtp_pass,
            None,
        )
        .expect("Failed to initialize SMTP service"),
    );

    let app_state = AppState::new(
        repos.user_repo,
        repos.pending_user_repo,
        repos.refresh_token_repo,
        repos.pending_change_repo,
        repos.measurement_repo,
        repos.exercise_repo,
        cripto_service,
        hmac_sha_service,
        jwt_service,
        lettre_service,
        r2_service,
        env.refresh_days,
    );

    let app = Router::new()
        .route("/", axum::routing::get(root))
        .merge(build_http())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(app_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("INFO server: running on {}", addr);

    println!("API documentation in: http://localhost:3000/swagger-ui");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
