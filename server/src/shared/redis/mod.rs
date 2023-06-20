use std::{sync::Arc, time::Duration};

use deadpool_redis::{Config, PoolConfig, Runtime, Timeouts};

pub type AsyncRedisConnection = deadpool_redis::redis::aio::Connection;
pub type AsyncRedisPool = deadpool_redis::Pool;
pub type ManagedAsyncRedisConnection = deadpool_redis::Connection;

pub fn build_connection_pool(url: &str) -> AsyncRedisPool {
    let max_size = 5;
    let timeouts = Timeouts {
        wait: Some(Duration::from_millis(5000)),
        create: Some(Duration::from_millis(5000)),
        recycle: Some(Duration::from_millis(5000)),
    };

    let mut cfg = Config::from_url(url);
    cfg.pool = Some(PoolConfig { max_size, timeouts });

    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}

pub async fn get_connection_from_pool(
    pool: &Arc<AsyncRedisPool>,
) -> Result<ManagedAsyncRedisConnection, AsyncRedisPoolError> {
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
