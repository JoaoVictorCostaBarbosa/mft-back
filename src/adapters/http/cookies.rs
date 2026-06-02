use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use cookie::time::{Duration, OffsetDateTime};

pub const ACCESS_TOKEN_COOKIE: &str = "mft_access_token";
pub const REFRESH_TOKEN_COOKIE: &str = "mft_refresh_token";

#[derive(Clone)]
pub struct CookieConfig {
    secure: bool,
    access_max_age: Duration,
    refresh_max_age: Duration,
}

impl CookieConfig {
    pub fn new(app_development: bool, access_minutes: i64, refresh_days: i64) -> Self {
        Self {
            secure: !app_development,
            access_max_age: Duration::minutes(access_minutes),
            refresh_max_age: Duration::days(refresh_days),
        }
    }

    pub fn add_auth_cookies(
        &self,
        jar: CookieJar,
        access_token: String,
        refresh_token: String,
    ) -> CookieJar {
        jar.add(self.auth_cookie(ACCESS_TOKEN_COOKIE, access_token, self.access_max_age))
            .add(self.auth_cookie(REFRESH_TOKEN_COOKIE, refresh_token, self.refresh_max_age))
    }

    pub fn clear_auth_cookies(&self, jar: CookieJar) -> CookieJar {
        jar.add(self.expired_cookie(ACCESS_TOKEN_COOKIE))
            .add(self.expired_cookie(REFRESH_TOKEN_COOKIE))
    }

    fn auth_cookie(&self, name: &'static str, value: String, max_age: Duration) -> Cookie<'static> {
        let expires_at = OffsetDateTime::now_utc() + max_age;

        Cookie::build((name, value))
            .path("/")
            .http_only(true)
            .secure(self.secure)
            .same_site(SameSite::Lax)
            .max_age(max_age)
            .expires(expires_at)
            .build()
    }

    fn expired_cookie(&self, name: &'static str) -> Cookie<'static> {
        Cookie::build((name, ""))
            .path("/")
            .http_only(true)
            .secure(self.secure)
            .same_site(SameSite::Lax)
            .max_age(Duration::ZERO)
            .expires(OffsetDateTime::UNIX_EPOCH)
            .build()
    }
}
