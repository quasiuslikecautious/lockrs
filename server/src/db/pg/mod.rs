pub mod models;
pub mod repositories;
pub mod schema;

use diesel_async::{
    pooled_connection::deadpool::{Object, Pool},
    AsyncPgConnection,
};

pub type AsyncPgPool = Pool<AsyncPgConnection>;
pub type ManagedAsyncPgConnection = Object<AsyncPgConnection>;
