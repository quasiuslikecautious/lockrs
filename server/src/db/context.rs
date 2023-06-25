use std::{pin::Pin, sync::Arc, time::Duration};

use deadpool::managed::Timeouts;
use deadpool_runtime::Runtime;
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    scoped_futures::ScopedFuture,
    AsyncPgConnection,
};

use deadpool_redis::{Config, PoolConfig};

type AsyncPgPool = Pool<AsyncPgConnection>;
type AsyncRedisPool = deadpool_redis::Pool;

pub type AsyncRedisConnection = deadpool_redis::redis::aio::Connection;

pub type ManagedAsyncRedisConnection = deadpool_redis::Connection;
pub type ManagedAsyncPgConnection = Object<AsyncPgConnection>;

type AsyncOutput<'target, T> = Pin<Box<dyn ScopedFuture<'target, 'target, Output = T> + Send>>;

pub struct DbContext {
    pg_pool: Arc<AsyncPgPool>,
    redis_pool: Arc<AsyncRedisPool>,
}

impl DbContext {
    pub async fn new(
        pg_url: &str,
        pg_pool_size: usize,
        redis_url: &str,
        redis_pool_size: usize,
    ) -> Self {
        Self {
            pg_pool: Arc::new(Self::create_pg_pool(pg_url, &pg_pool_size)),
            redis_pool: Arc::new(Self::create_redis_pool(redis_url, &redis_pool_size)),
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
            .clone()
            .as_ref()
            .get()
            .await
            .map_err(|_| DbContextError::ConnectionFailed)
    }

    pub async fn get_redis_connection(
        &self,
    ) -> Result<ManagedAsyncRedisConnection, DbContextError> {
        self.redis_pool
            .clone()
            .as_ref()
            .get()
            .await
            .map_err(|_| DbContextError::ConnectionFailed)
    }

    pub async fn execute_in_pg_transaction<F, T>(&self, f: F) -> Result<T, DbContextError>
    where
        F: FnOnce(&mut AsyncPgConnection) -> AsyncOutput<Result<T, diesel::result::Error>>
            + Send
            + 'static,
        T: Send,
    {
        let connection = &mut self.get_pg_connection().await?;
        connection
            .build_transaction()
            .read_write()
            .run(|conn| {
                let future = f(conn);
                Box::pin(future)
            })
            .await
            .map_err(|_| DbContextError::BadTransaction)
    }
}

pub enum DbContextError {
    ConnectionFailed,
    BadTransaction,
}
