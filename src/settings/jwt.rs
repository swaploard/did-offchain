use once_cell::sync::Lazy;
use std::env;

pub struct JwtConfig {
    pub access_secret: String,
    pub refresh_secret: String,
    pub access_expiry_minutes: i64,
    pub refresh_expiry_minutes: i64,
}

pub static JWT_CONFIG: Lazy<JwtConfig> = Lazy::new(|| {
    let access_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in environment variables");
    let refresh_secret = env::var("REFRESH_SECRET")
        .expect("REFRESH_SECRET must be set in environment variables");

    // e.g. access token valid for 15 minutes, refresh token for 7 days:
    let access_expiry_minutes = 15;            // 15 minutes
    let refresh_expiry_minutes = 60 * 24 * 7;   // 7 days in minutes

    JwtConfig {
        access_secret,
        refresh_secret,
        access_expiry_minutes,
        refresh_expiry_minutes,
    }
});
