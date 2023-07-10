use std::time::Duration;

use deadpool::managed::Timeouts;
use deadpool_redis::{Config, PoolConfig};
use deadpool_runtime::Runtime;
use diesel_async::pooled_connection::{deadpool::{Object, Pool}, AsyncDieselConnectionManager};

type AsyncPgPool = Pool<AsyncPgConnection>;
type AsyncRedisPool = deadpool_redis::Pool;

pub type AsyncPgConnection = diesel_async::AsyncPgConnection;
pub type AsyncRedisConnection = deadpool_redis::redis::aio::Connection;

pub type ManagedAsyncPgConnection = Object<AsyncPgConnection>;
pub type ManagedAsyncRedisConnection = deadpool_redis::Connection;

pub struct DbContext {
    pg_pool: AsyncPgPool,
    redis_pool: AsyncRedisPool,
}

impl DbContext {
    pub async fn new(
        pg_url: &str,
        pg_pool_size: usize,
        redis_url: &str,
        redis_pool_size: usize,
    ) -> Self {
        Self {
            pg_pool: Self::create_pg_pool(pg_url, &pg_pool_size),
            redis_pool: Self::create_redis_pool(redis_url, &redis_pool_size),
        }
    }

    fn create_pg_pool(url: &str, pool_size: &usize) -> AsyncPgPool {
        let pg_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);

        Pool::builder(pg_manager)
            .max_size(*pool_size)
            .runtime(Runtime::Tokio1)
            .timeouts(Timeouts::wait_millis(5000))
            .build()
            .expect("Could not build pg connection pool")
    }

    fn create_redis_pool(url: &str, pool_size: &usize) -> AsyncRedisPool {
        let timeouts = Timeouts {
            wait: Some(Duration::from_millis(5000)),
            create: Some(Duration::from_millis(5000)),
            recycle: Some(Duration::from_millis(5000)),
        };

        let mut cfg = Config::from_url(url);
        cfg.pool = Some(PoolConfig {
            max_size: *pool_size,
            timeouts,
        });

        cfg.create_pool(Some(Runtime::Tokio1))
            .expect("Could not build redis connection pool")
    }

    pub async fn get_pg_connection(&self) -> Result<ManagedAsyncPgConnection, DbContextError> {
        self.pg_pool
            .get()
            .await
            .map_err(|_| DbContextError::ConnectionFailed)
    }

    pub async fn get_redis_connection(
        &self,
    ) -> Result<ManagedAsyncRedisConnection, DbContextError> {
        self.redis_pool
            .get()
            .await
            .map_err(|_| DbContextError::ConnectionFailed)
    } 
}

#[derive(Debug)]
pub enum DbContextError {
    ConnectionFailed,
    BadTransaction,
}

impl From<diesel::result::Error> for DbContextError {
    fn from(_error: diesel::result::Error) -> Self {
        Self::BadTransaction
    }
}
