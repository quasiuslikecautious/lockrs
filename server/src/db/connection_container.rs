use std::sync::Arc;

use diesel_async::scoped_futures::ScopedBoxFuture;

use crate::{AsyncPgConnection, AsyncRedisConnection, DbContext, ManagedAsyncPgConnection};

pub struct ConnectionContainer {
    pg: Option<ManagedAsyncPgConnection>,
    redis: Option<AsyncRedisConnection>,
    in_transaction: bool,
}

impl ConnectionContainer {
    pub fn new() -> Self {
        Self {
            pg: None,
            redis: None,
            in_transaction: false,
        }
    }

    pub async fn get_pg(
        &mut self,
        db_context: &Arc<DbContext>,
    ) -> Result<&mut ManagedAsyncPgConnection, ConnectionError> {
        Ok(self.pg.get_or_insert(
            db_context
                .as_ref()
                .get_pg_connection()
                .await
                .map_err(|_| ConnectionError::Pool)?,
        ))
    }

    pub async fn execute_in_pg_transaction<'b, T, F>(
        &mut self,
        db_context: &Arc<DbContext>,
        f: F,
    ) -> Result<T, ConnectionError>
    where
        F: for<'r> FnOnce(
                &'r mut AsyncPgConnection,
            ) -> ScopedBoxFuture<'b, 'r, Result<T, ConnectionError>>
            + Send,
        T: 'b,
    {
        let in_transaction = std::mem::replace(&mut self.in_transaction, true);

        if in_transaction {
            let conn = self.get_pg(db_context).await?;
            let result = f(conn)
                .await
                .map_err(|_| ConnectionError::Transaction);

            return result;
        }

        let connection = self.get_pg(db_context).await?;

        let result = connection
            .build_transaction()
            .read_write()
            .run(|conn| {
                f(conn)
            })
            .await
            .map_err(|_| ConnectionError::Transaction);

        result
    }

    pub async fn get_redis(&mut self) -> AsyncRedisConnection {
        todo!();
    }
}

pub enum ConnectionError {
    Pool,
    Transaction,
}

impl From<diesel::result::Error> for ConnectionError {
    fn from(_: diesel::result::Error) -> Self {
        Self::Transaction
    }
}
