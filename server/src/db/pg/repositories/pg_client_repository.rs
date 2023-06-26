use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, RunQueryDsl};
use uuid::Uuid;

use crate::{
    mappers::ClientMapper,
    models::{ClientModel, ClientUpdateModel, RedirectCreateModel},
    pg::{models::PgClient, schema::clients},
    repositories::{ClientRepository, ClientRepositoryError, RedirectUriRepository},
    DbContext,
};

pub struct PgClientRepository {
    db_context: Arc<DbContext>,
}

impl PgClientRepository {
    pub fn new(db_context: &Arc<DbContext>) -> Self {
        Self {
            db_context: Arc::clone(db_context),
        }
    }
}

#[async_trait]
impl ClientRepository for PgClientRepository {
    async fn create(
        &self,
        redirect_repo: &Box<dyn RedirectUriRepository>,
        client_create: &ClientModel,
        redirect_create: &RedirectCreateModel,
    ) -> Result<ClientModel, ClientRepositoryError> {
        let pg_client = self
            .db_context
            .as_ref()
            .execute_in_pg_transaction(|conn| {
                async move {
                    let client = diesel::insert_into(clients::table)
                        .values((
                            clients::id.eq(&client_create.id),
                            clients::secret.eq(&client_create.secret),
                            clients::user_id.eq(&client_create.user_id),
                            clients::is_public.eq(client_create.secret.is_some()),
                            clients::name.eq(&client_create.name),
                            clients::description.eq(&client_create.description),
                            clients::homepage_url.eq(&client_create.homepage_url.to_string()),
                        ))
                        .get_result::<PgClient>(conn)
                        .await?;

                    redirect_repo
                        .create(redirect_create)
                        .await
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                    Ok(client)
                }
                .scope_boxed()
            })
            .await
            .map_err(|_| ClientRepositoryError::NotCreated)?;

        Ok(ClientMapper::from_pg(pg_client))
    }

    async fn get_by_id(&self, id: &str) -> Result<ClientModel, ClientRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| ClientRepositoryError::BadConnection)?;

        let pg_client = clients::table
            .filter(clients::id.eq(id))
            .first::<PgClient>(conn)
            .await
            .map_err(|_| ClientRepositoryError::NotFound)?;

        Ok(ClientMapper::from_pg(pg_client))
    }

    async fn get_by_credentials(
        &self,
        id: &str,
        secret: &Option<String>,
    ) -> Result<ClientModel, ClientRepositoryError> {
        let mut query = clients::table
            .into_boxed()
            .filter(clients::id.eq(&id))
            .filter(clients::is_public.eq(secret.is_some()));

        if let Some(secret) = secret {
            query = query.filter(clients::secret.eq(secret));
        }

        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| ClientRepositoryError::BadConnection)?;

        let pg_client = query
            .first::<PgClient>(conn)
            .await
            .map_err(|_| ClientRepositoryError::NotFound)?;

        Ok(ClientMapper::from_pg(pg_client))
    }

    async fn get_all_by_user_id(
        &self,
        id: &Uuid,
    ) -> Result<Vec<ClientModel>, ClientRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| ClientRepositoryError::BadConnection)?;

        let clients = clients::table
            .filter(clients::user_id.eq(id))
            .load::<PgClient>(conn)
            .await
            .map_err(|_| ClientRepositoryError::NotFound)?;

        Ok(clients
            .into_iter()
            .map(ClientMapper::from_pg)
            .collect::<Vec<ClientModel>>())
    }

    async fn update_by_id(
        &self,
        id: &str,
        client_update: &ClientUpdateModel,
    ) -> Result<ClientModel, ClientRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| ClientRepositoryError::BadConnection)?;

        let pg_client = diesel::update(clients::table)
            .filter(clients::id.eq(id))
            .set(client_update)
            .get_result::<PgClient>(conn)
            .await
            .map_err(|_| ClientRepositoryError::NotUpdated)?;

        Ok(ClientMapper::from_pg(pg_client))
    }

    async fn delete_by_id(&self, id: &str) -> Result<(), ClientRepositoryError> {
        let conn = &mut self
            .db_context
            .get_pg_connection()
            .await
            .map_err(|_| ClientRepositoryError::BadConnection)?;

        let affected_rows = diesel::delete(clients::table)
            .filter(clients::id.eq(id))
            .execute(conn)
            .await
            .map_err(|_| ClientRepositoryError::BadDelete)?;

        if affected_rows != 1 {
            return Err(ClientRepositoryError::BadDelete);
        }

        Ok(())
    }
}
