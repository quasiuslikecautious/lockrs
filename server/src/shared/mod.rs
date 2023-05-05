pub mod db;
pub mod mappers;
pub mod models;
pub mod redis;
pub mod services;
pub mod utils;

use std::sync::Arc;

use chrono::Duration;

use self::{
    db::AsyncPgPool,
    redis::AsyncRedisPool,
    utils::jwt::{JwtUtil, RotatingKey},
};

pub struct AppState {
    pub jwt_util: Arc<JwtUtil>,
    pub db_pool: Arc<AsyncPgPool>,
    pub redis_pool: Arc<AsyncRedisPool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            jwt_util: Arc::new(JwtUtil {
                secret: RotatingKey::new(Duration::minutes(2), Duration::minutes(1)),
            }),
            db_pool: Arc::new(db::build_connection_pool()),
            redis_pool: Arc::new(redis::build_connection_pool()),
        }
    }
}
