use dotenvy::dotenv;
use sqlx::PgPool;

pub async fn create_pool(database_url: &str) -> PgPool {
    dotenv().ok();
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}
