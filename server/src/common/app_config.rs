use std::env;

use chrono::Duration;
use dotenvy::dotenv;

pub struct AppConfig {
    pub postgres_url: String,
    pub redis_url: String,
    pub key_interval: Duration,
    pub auth_interval: Duration,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let postgres_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set!");

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set!");

        let key_interval_sec = env::var("KEY_INTERVAL")
            .expect("KEY_INTERVAL must be set!")
            .parse::<i64>()
            .expect("KEY_INTERVAL must be an i64!");
        let key_interval = Duration::seconds(key_interval_sec);

        let auth_interval_sec = env::var("AUTH_INTERVAL")
            .expect("AUTH_INTERVAL must be set!")
            .parse::<i64>()
            .expect("AUTH_INTERVAL must be an i64!");
        let auth_interval = Duration::seconds(auth_interval_sec);

        Self {
            postgres_url,
            redis_url,
            key_interval,
            auth_interval,
        }
    }
}
