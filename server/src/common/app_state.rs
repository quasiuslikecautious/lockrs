use std::sync::Arc;

use crate::{
    pg, redis,
    utils::jwt::{JwtUtil, RotatingKey},
    AppConfig,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub jwt_util: Arc<JwtUtil>,
    pub db_pool: Arc<pg::AsyncPgPool>,
    pub redis_pool: Arc<redis::AsyncRedisPool>,
}

impl AppState {
    pub fn new() -> Self {
        let config = Arc::new(AppConfig::new());

        let key_duration = config.as_ref().key_interval;
        let overlap_duration = config.as_ref().auth_interval;
        let postgres_url = config.as_ref().postgres_url.clone();
        let redis_url = config.as_ref().redis_url.clone();

        Self {
            config,
            jwt_util: Arc::new(JwtUtil {
                secret: RotatingKey::new(&key_duration, &overlap_duration),
            }),
            db_pool: Arc::new(pg::build_connection_pool(postgres_url.as_str())),
            redis_pool: Arc::new(redis::build_connection_pool(redis_url.as_str())),
        }
    }
}
