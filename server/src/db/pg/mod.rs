pub mod models;
pub mod repositories;
pub mod schema;

use std::sync::Arc;

use deadpool::managed::Timeouts;
use deadpool_runtime::Runtime;
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};

pub type AsyncPgPool = Pool<AsyncPgConnection>;
pub type ManagedAsyncPgConnection = Object<AsyncPgConnection>;

pub fn build_connection_pool(url: &str) -> AsyncPgPool {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);

    Pool::builder(manager)
        .max_size(5)
        .runtime(Runtime::Tokio1)
        .timeouts(Timeouts::wait_millis(5000))
        .build()
        .expect("Could not build db connection pool")
}

pub async fn get_connection_from_pool(
    db_pool: &Arc<AsyncPgPool>,
) -> Result<ManagedAsyncPgConnection, AsyncPgPoolError> {
    let managed_conn = db_pool
        .clone()
        .as_ref()
        .get()
        .await
        .map_err(|_| AsyncPgPoolError::Timeout)?;

    Ok(managed_conn)
}

pub enum AsyncPgPoolError {
    Timeout,
}
