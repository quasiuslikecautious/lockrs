use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use url::Url;

use crate::{
    mappers::RedirectMapper,
    models::{RedirectCreateModel, RedirectModel},
    pg::{models::PgRedirectUri, schema::redirect_uris},
    repositories::{RedirectUriRepository, RedirectUriRepositoryError},
    DbContext,
};

pub struct PgRedirectUriRepository;

#[async_trait]
impl RedirectUriRepository for PgRedirectUriRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        redirect_create: &RedirectCreateModel,
    ) -> Result<RedirectModel, RedirectUriRepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RedirectUriRepositoryError::BadConnection)?;

        let pg_redirect = diesel::insert_into(redirect_uris::table)
            .values((
                redirect_uris::client_id.eq(&redirect_create.client_id),
                redirect_uris::uri.eq(redirect_create.uri.to_string()),
            ))
            .get_result::<PgRedirectUri>(conn)
            .await
            .map_err(|err| {
                println!("{:?}", err);
                RedirectUriRepositoryError::NotCreated
            })?;

        println!("successfully created redirect...");

        Ok(RedirectMapper::from_pg(pg_redirect))
    }

    async fn get_by_uri(
        &self,
        db_context: &Arc<DbContext>,
        client_id: &str,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectUriRepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RedirectUriRepositoryError::BadConnection)?;

        let db_redirect = redirect_uris::table
            .filter(redirect_uris::client_id.eq(client_id))
            .filter(redirect_uris::uri.eq(uri.to_string()))
            .first::<PgRedirectUri>(conn)
            .await
            .map_err(|_| RedirectUriRepositoryError::NotFound)?;

        Ok(RedirectMapper::from_pg(db_redirect))
    }

    async fn get_all_by_client_id(
        &self,
        db_context: &Arc<DbContext>,
        client_id: &str,
    ) -> Result<Vec<RedirectModel>, RedirectUriRepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RedirectUriRepositoryError::BadConnection)?;

        let db_redirects = redirect_uris::table
            .filter(redirect_uris::client_id.eq(client_id))
            .load::<PgRedirectUri>(conn)
            .await
            .map_err(|_| RedirectUriRepositoryError::NoneFound)?;

        Ok(db_redirects
            .into_iter()
            .map(RedirectMapper::from_pg)
            .collect::<Vec<RedirectModel>>())
    }
}
