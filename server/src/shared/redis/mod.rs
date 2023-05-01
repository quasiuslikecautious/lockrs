use std::{env, sync::Arc};

use deadpool_redis::{
    Config, Runtime,
};
use dotenvy::dotenv;

pub type AsyncRedisConnection = deadpool_redis::Connection;
pub type AsyncRedisPool = deadpool_redis::Pool;
pub type ManagedAsyncRedisConnection = deadpool::managed::Object<AsyncRedisConnection>;

pub fn redis_url_for_env() -> String {
    dotenv().ok();

    env::var("REDIS_URL").expect("REDIS_URL must be set!")
}

pub fn build_connection_pool() -> AsyncRedisPool {
    let _url = redis_url_for_env();
    let cfg = Config::from_url(redis_url_for_env().as_str());

    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}

pub async fn get_connection_from_pool(
    pool: &Arc<AsyncRedisPool>,
) -> Result<AsyncRedisConnection, AsyncRedisPoolError> {
    let managed_conn = pool
        .clone()
        .as_ref()
        .get()
        .await
        .map_err(|_| AsyncRedisPoolError::Timeout)?;

    Ok(managed_conn)
}

pub enum AsyncRedisPoolError {
    Timeout,
}
