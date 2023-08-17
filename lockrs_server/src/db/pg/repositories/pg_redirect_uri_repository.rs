use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use url::Url;
use uuid::Uuid;

use crate::{
    db::{
        pg::{
            models::{PgClient, PgRedirectUri},
            schema::{clients, redirect_uris},
        },
        repositories::{QueryFailure, RedirectUriRepository, RepositoryError},
        DbContext,
    },
    mappers::RedirectMapper,
    models::{RedirectCreateModel, RedirectModel},
};

pub struct PgRedirectUriRepository;

#[async_trait]
impl RedirectUriRepository for PgRedirectUriRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        redirect_create: &RedirectCreateModel,
    ) -> Result<RedirectModel, RepositoryError> {
        tracing::trace!(
            method = "create",
            redirect = ?redirect_create
        );

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let pg_redirect = diesel::insert_into(redirect_uris::table)
            .values((
                redirect_uris::client_id.eq(&redirect_create.client_id),
                redirect_uris::uri.eq(redirect_create.uri.to_string()),
            ))
            .get_result::<PgRedirectUri>(conn)
            .await
            .map_err(RepositoryError::map_diesel_create)?;

        Ok(RedirectMapper::from_pg(pg_redirect))
    }

    async fn get_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<RedirectModel, RepositoryError> {
        tracing::trace!(method = "get_by_id", ?id);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let pg_redirect = redirect_uris::table
            .filter(redirect_uris::id.eq(id))
            .first::<PgRedirectUri>(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(RedirectMapper::from_pg(pg_redirect))
    }

    async fn get_by_uri(
        &self,
        db_context: &Arc<DbContext>,
        client_id: &str,
        uri: &Url,
    ) -> Result<RedirectModel, RepositoryError> {
        tracing::trace!(method = "get_by_uri", client_id, ?uri);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let db_redirect = redirect_uris::table
            .filter(redirect_uris::client_id.eq(client_id))
            .filter(redirect_uris::uri.eq(uri.to_string()))
            .first::<PgRedirectUri>(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(RedirectMapper::from_pg(db_redirect))
    }

    async fn get_user_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<Uuid, RepositoryError> {
        tracing::trace!(method = "get_user_id", ?id);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let (_db_redirect, db_client) = redirect_uris::table
            .inner_join(clients::table)
            .filter(redirect_uris::id.eq(id))
            .select((PgRedirectUri::as_select(), PgClient::as_select()))
            .first::<(PgRedirectUri, PgClient)>(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(db_client.user_id)
    }

    async fn get_all_by_client_id(
        &self,
        db_context: &Arc<DbContext>,
        client_id: &str,
    ) -> Result<Vec<RedirectModel>, RepositoryError> {
        tracing::trace!(method = "get_all_by_client_id", client_id);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let db_redirects = redirect_uris::table
            .filter(redirect_uris::client_id.eq(client_id))
            .load::<PgRedirectUri>(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(db_redirects
            .into_iter()
            .map(RedirectMapper::from_pg)
            .collect::<Vec<RedirectModel>>())
    }

    async fn delete_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<(), RepositoryError> {
        tracing::trace!(method = "delete_by_id", ?id);

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let affected_rows = diesel::delete(redirect_uris::table)
            .filter(redirect_uris::id.eq(id))
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
}
