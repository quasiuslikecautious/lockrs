use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::{
        pg::{
            models::{PgClient, PgRedirectUri},
            schema::clients,
            schema::redirect_uris,
        },
        repositories::{ClientRepository, RepositoryError},
        DbContext,
    },
    mappers::ClientMapper,
    models::{ClientModel, ClientUpdateModel, RedirectCreateModel},
};

pub struct PgClientRepository;

#[async_trait]
impl ClientRepository for PgClientRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        client_create: &ClientModel,
        redirect_create: &RedirectCreateModel,
    ) -> Result<ClientModel, RepositoryError> {
        let connection = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| {
                let msg = format!("TODO");
                RepositoryError::ConnectionFailed(msg)
            })?;


        let pg_client = connection
            .transaction::<_, diesel::result::Error, _>(|conn| {
                async move {
                    let client = diesel::insert_into(clients::table)
                        .values((
                            clients::id.eq(&client_create.id),
                            clients::secret.eq(&client_create.secret),
                            clients::user_id.eq(&client_create.user_id),
                            clients::is_public.eq(client_create.secret.is_none()),
                            clients::name.eq(&client_create.name),
                            clients::description.eq(&client_create.description),
                            clients::homepage_url.eq(&client_create.homepage_url.to_string()),
                        ))
                        .get_result::<PgClient>(conn)
                        .await?;

                    diesel::insert_into(redirect_uris::table)
                        .values((
                            redirect_uris::client_id.eq(&redirect_create.client_id),
                            redirect_uris::uri.eq(redirect_create.uri.to_string()),
                        ))
                        .get_result::<PgRedirectUri>(conn)
                        .await?;

                    Ok(client)
                }
                .scope_boxed()
            })
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotCreated(msg)
            })?;

        Ok(ClientMapper::from_pg(pg_client))
    }

    async fn get_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<ClientModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| {
                let msg = format!("TODO");
                RepositoryError::ConnectionFailed(msg)
            })?;

        let pg_client = clients::table
            .filter(clients::id.eq(id))
            .first::<PgClient>(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotFound(msg)
            })?;

        Ok(ClientMapper::from_pg(pg_client))
    }

    async fn get_by_credentials(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
        secret: Option<&str>,
    ) -> Result<ClientModel, RepositoryError> {
        let mut query = clients::table
            .into_boxed()
            .filter(clients::id.eq(&id))
            .filter(clients::is_public.eq(secret.is_none()));

        if let Some(secret) = secret {
            query = query.filter(clients::secret.eq(secret));
        }

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| {
                let msg = format!("TODO");
                RepositoryError::ConnectionFailed(msg)
            })?;

        let pg_client = query
            .first::<PgClient>(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotFound(msg)
            })?;

        Ok(ClientMapper::from_pg(pg_client))
    }

    async fn get_all_by_user_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<Vec<ClientModel>, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| {
                let msg = format!("TODO");
                RepositoryError::ConnectionFailed(msg)
            })?;

        let clients = clients::table
            .filter(clients::user_id.eq(id))
            .load::<PgClient>(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotFound(msg)
            })?;

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
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| {
                let msg = format!("TODO");
                RepositoryError::ConnectionFailed(msg)
            })?;

        let pg_client = diesel::update(clients::table)
            .filter(clients::id.eq(id))
            .set(client_update)
            .get_result::<PgClient>(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotUpdated(msg)
            })?;

        Ok(ClientMapper::from_pg(pg_client))
    }

    async fn delete_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<(), RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| {
                let msg = format!("TODO");
                RepositoryError::ConnectionFailed(msg)
            })?;

        let affected_rows = diesel::delete(clients::table)
            .filter(clients::id.eq(id))
            .execute(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotDeleted(msg)
            })?;

        if affected_rows != 1 {
            let msg = format!("Expected 1 row to be affected by delete, but found {}", affected_rows);
            return Err(RepositoryError::NotDeleted(msg));
        }

        Ok(())
    }
}
