use std::sync::Arc;

use async_trait::async_trait;
use chrono::offset::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    db::{
        pg::{models::PgAccessToken, schema::access_tokens},
        repositories::{AccessTokenRepository, RepositoryError},
        DbContext,
    },
    oauth2::{
        mappers::AccessTokenMapper,
        models::{AccessTokenCreateModel, AccessTokenModel},
    },
};

pub struct PgAccessTokenRepository;

#[async_trait]
impl AccessTokenRepository for PgAccessTokenRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        token_create: &AccessTokenCreateModel,
    ) -> Result<AccessTokenModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::ConnectionFailed(msg)
            })?;

        let pg_token = diesel::insert_into(access_tokens::table)
            .values((
                access_tokens::token.eq(&token_create.token),
                access_tokens::client_id.eq(&token_create.client_id),
                access_tokens::user_id.eq(&token_create.user_id),
                access_tokens::expires_at.eq(&token_create.expires_at),
                access_tokens::scopes.eq(&token_create.scopes),
            ))
            .get_result::<PgAccessToken>(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotCreated(msg)
            })?;

        Ok(AccessTokenMapper::from_pg(pg_token))
    }

    async fn get_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<AccessTokenModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::ConnectionFailed(msg)
            })?;

        let now = Utc::now().naive_utc();

        let pg_token = access_tokens::table
            .filter(access_tokens::token.eq(token))
            .filter(access_tokens::created_at.lt(&now))
            .filter(access_tokens::expires_at.gt(&now))
            .first::<PgAccessToken>(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotFound(msg)
            })?;

        Ok(AccessTokenMapper::from_pg(pg_token))
    }

    async fn delete_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<(), RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::ConnectionFailed(msg)
            })?;

        let affected_rows = diesel::delete(access_tokens::table)
            .filter(access_tokens::token.eq(token))
            .execute(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotFound(msg)
            })?;

        if affected_rows != 1 {
            let msg = format!(
                "Expected 1 row to be affected by delete, but found {}",
                affected_rows
            );
            return Err(RepositoryError::NotDeleted(msg));
        }

        Ok(())
    }
}
