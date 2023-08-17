use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use scoped_futures::ScopedFutureExt;
use uuid::Uuid;

use crate::{
    db::{
        pg::{models::PgClient, schema::clients},
        repositories::{ClientRepository, QueryFailure, RepositoryError},
        DbContext,
    },
    mappers::ClientMapper,
    models::{ClientModel, ClientUpdateModel},
};

pub struct PgClientRepository;

#[async_trait]
impl ClientRepository for PgClientRepository {
    async fn get_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<ClientModel, RepositoryError> {
        tracing::trace!(method = "get_by_id", id);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let pg_client = clients::table
            .filter(clients::id.eq(id))
            .first::<PgClient>(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(ClientMapper::from_pg(pg_client))
    }

    async fn get_all_by_user_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<Vec<ClientModel>, RepositoryError> {
        tracing::trace!(method = "get_all_by_user_id", ?id);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let clients = clients::table
            .filter(clients::user_id.eq(id))
            .load::<PgClient>(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(clients
            .into_iter()
            .map(ClientMapper::from_pg)
            .collect::<Vec<ClientModel>>())
    }

    async fn update_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
        client_update: &ClientUpdateModel,
    ) -> Result<ClientModel, RepositoryError> {
        tracing::trace!(
            method = "update_by_id",
            id,
            client = ?client_update
        );

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let pg_client = diesel::update(clients::table)
            .filter(clients::id.eq(id))
            .set(client_update)
            .get_result::<PgClient>(conn)
            .await
            .map_err(RepositoryError::map_diesel_update)?;

        Ok(ClientMapper::from_pg(pg_client))
    }

    async fn delete_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<(), RepositoryError> {
        tracing::trace!(method = "delete_by_id", id);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        conn.transaction::<(), RepositoryError, _>(|conn| {
            async move {
                let affected_rows = diesel::delete(clients::table)
                    .filter(clients::id.eq(id))
                    .execute(conn)
                    .await
                    .map_err(RepositoryError::map_diesel_delete)?;

                if affected_rows != 1 {
                    let msg = format!(
                        "Expected 1 row to be affected by delete, but found {}",
                        affected_rows
                    );

                    tracing::error!(error = msg);
                    return Err(RepositoryError::QueryFailed(QueryFailure::NotDeleted));
                }

                Ok(())
            }
            .scope_boxed()
        })
        .await
    }
}
