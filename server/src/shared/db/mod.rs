pub mod models;
pub mod schema;

mod db_error;
pub use self::db_error::*;

use deadpool::managed::Timeouts;
use deadpool_runtime::Runtime;
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use dotenvy::dotenv;
use std::{env, sync::Arc};

pub type AsyncPgPool = Pool<AsyncPgConnection>;
pub type ManagedAsyncPgConnection = Object<AsyncPgConnection>;

pub fn database_url_for_env() -> String {
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn build_connection_pool() -> Pool<AsyncPgConnection> {
    let url = database_url_for_env();
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);

    Pool::builder(manager)
        .max_size(10)
        .runtime(Runtime::Tokio1)
        .timeouts(Timeouts::wait_millis(5000))
        .build()
        .expect("Could not build connection pool")
}

pub async fn get_connection_from_pool(
    db_pool: &Arc<AsyncPgPool>,
) -> Result<ManagedAsyncPgConnection, AsyncPoolError> {
    let managed_conn = db_pool
        .clone()
        .as_ref()
        .get()
        .await
        .map_err(|_| AsyncPoolError::Timeout)?;

    Ok(managed_conn)
}

pub enum AsyncPoolError {
    Timeout,
}
