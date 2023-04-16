pub mod db;
pub mod mappers;
pub mod models;
pub mod services;
pub mod utils;

use std::sync::Arc;

use self::db::{build_connection_pool, AsyncPgPool};

pub struct AppState {
    pub db_pool: Arc<AsyncPgPool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db_pool: Arc::new(build_connection_pool()),
        }
    }
}
