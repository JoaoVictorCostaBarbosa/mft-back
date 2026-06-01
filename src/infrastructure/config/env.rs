use std::env;

pub struct LoadEnv {
    pub smtp_from: Option<String>,
    pub port: u16,
    pub app_development: bool,
    pub cors_allowed_origins: Vec<String>,

    pub database_url: String,

    pub secret_access_key: String,
    pub secret_refresh_key: String,

    pub access_minutes: i64,
    pub refresh_days: i64,

    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_secure: bool,
    pub smtp_user: String,
    pub smtp_pass: String,

    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
    pub r2_bucket_name: String,
    pub r2_s3_endpoint: String,
    pub r2_public_base_url: String,
}

impl LoadEnv {
    pub fn new() -> Self {
        Self {
            smtp_from: env::var("SMTP_FROM").ok(),
            port: env_var_parse("PORT"),
            app_development: env_var_parse_bool_default("APP_DEVELOPMENT", false),
            cors_allowed_origins: env_var_list_default(
                "CORS_ALLOWED_ORIGINS",
                vec!["http://localhost:5173".to_string()],
            ),

            database_url: env_var("DATABASE_URL"),

            secret_access_key: env_var("SECRET_ACCESS_KEY"),
            secret_refresh_key: env_var("SECRET_REFRESH_KEY"),

            access_minutes: env_var_parse("ACCESS_MINUTES"),
            refresh_days: env_var_parse("REFRESH_DAYS"),

            smtp_host: env_var("SMTP_HOST"),
            smtp_port: env_var_parse("SMTP_PORT"),
            smtp_secure: env_var_parse_bool("SMTP_SECURE"),
            smtp_user: env_var("SMTP_USER"),
            smtp_pass: env_var("SMTP_PASS"),

            r2_access_key_id: env_var("R2_ACCESS_KEY_ID"),
            r2_secret_access_key: env_var("R2_SECRET_ACCESS_KEY"),
            r2_bucket_name: env_var("R2_BUCKET_NAME"),
            r2_s3_endpoint: env_var("R2_S3_ENDPOINT"),
            r2_public_base_url: env_var("R2_PUBLIC_BASE_URL"),
        }
    }
}

fn env_var(key: &str) -> String {
    env::var(key).expect(&format!("missing env var: {}", key))
}

fn env_var_parse<T: std::str::FromStr>(key: &str) -> T {
    env::var(key)
        .expect(&format!("missing env var: {}", key))
        .parse()
        .unwrap_or_else(|_| panic!("invalid value for env var: {}", key))
}

fn env_var_parse_bool(key: &str) -> bool {
    let v = env::var(key).expect(&format!("missing env var: {}", key));
    matches!(v.to_lowercase().as_str(), "true" | "1" | "yes")
}

fn env_var_parse_bool_default(key: &str, default: bool) -> bool {
    env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "true" | "1" | "yes"))
        .unwrap_or(default)
}

fn env_var_list_default(key: &str, default: Vec<String>) -> Vec<String> {
    env::var(key)
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|origin| !origin.is_empty())
                .map(ToString::to_string)
                .collect()
        })
        .ok()
        .filter(|values: &Vec<String>| !values.is_empty())
        .unwrap_or(default)
}
