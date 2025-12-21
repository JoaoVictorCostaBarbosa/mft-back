mod adapters;
mod api_doc;
mod application;
mod db;
mod domain;
mod infrastructure;
use crate::{
    api_doc::ApiDoc,
    application::app_state::app_state::AppState,
    domain::services::{cripto::CriptoService, jwt::JwtProvider},
    infrastructure::{
        config::env::LoadEnv,
        providers::{mail::lettre_sending::LettreSmtpService, r2_storage::R2Storage},
        repositories::postgres::RepositoryBundle,
        security::{argon2_hasher::Argon2Hasher, jwt::jwt_token_service::JwtService},
    },
    adapters::http::routers::build_http,
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

    let jwt_service: Arc<dyn JwtProvider> = Arc::new(JwtService::new(
        env.secret_access_key,
        env.secret_refresh_key,
        env.access_minutes,
        env.refresh_days,
    ));

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
        repos.pending_change_repo,
        repos.measurement_repo,
        cripto_service,
        jwt_service,
        lettre_service,
        r2_service,
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
