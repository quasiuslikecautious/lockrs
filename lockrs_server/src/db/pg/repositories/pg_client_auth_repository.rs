use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};

use crate::{
    db::{
        pg::{
            models::{PgClient, PgRedirectUri},
            schema::clients,
            schema::redirect_uris,
        },
        repositories::{ClientAuthRepository, RepositoryError},
        DbContext,
    },
    mappers::ClientAuthMapper,
    models::{ClientAuthModel, RedirectCreateModel},
};

pub struct PgClientAuthRepository;

#[async_trait]
impl ClientAuthRepository for PgClientAuthRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        client_create: &ClientAuthModel,
        redirect_create: &RedirectCreateModel,
    ) -> Result<ClientAuthModel, RepositoryError> {
        tracing::trace!(
            method = "create",
            user_id = ?client_create.user_id,
            id = client_create.id,
            redirect_uri = ?redirect_create.uri
        );

        let connection = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

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
            .map_err(RepositoryError::map_diesel_create)?;

        Ok(ClientAuthMapper::from_pg(pg_client))
    }

    async fn get_by_credentials(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
        secret: Option<&str>,
    ) -> Result<ClientAuthModel, RepositoryError> {
        tracing::trace!(method = "get_by_credentials", id);

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
            .map_err(RepositoryError::from)?;

        let pg_client = query
            .first::<PgClient>(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(ClientAuthMapper::from_pg(pg_client))
    }
}
