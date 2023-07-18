use std::sync::Arc;

use async_trait::async_trait;
use chrono::offset::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    db::{
        pg::{models::PgRefreshToken, schema::refresh_tokens},
        repositories::{RefreshTokenRepository, RepositoryError},
        DbContext,
    },
    oauth2::{
        mappers::RefreshTokenMapper,
        models::{RefreshTokenCreateModel, RefreshTokenModel},
    },
};

pub struct PgRefreshTokenRepository;

#[async_trait]
impl RefreshTokenRepository for PgRefreshTokenRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        token_create: &RefreshTokenCreateModel,
    ) -> Result<RefreshTokenModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::ConnectionFailed(msg)
            })?;

        let pg_token = diesel::insert_into(refresh_tokens::table)
            .values((
                refresh_tokens::token.eq(&token_create.token),
                refresh_tokens::client_id.eq(&token_create.client_id),
                refresh_tokens::user_id.eq(&token_create.user_id),
                refresh_tokens::expires_at.eq(&token_create.expires_at),
                refresh_tokens::scopes.eq(&token_create.scopes),
            ))
            .get_result::<PgRefreshToken>(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotCreated(msg)
            })?;

        Ok(RefreshTokenMapper::from_pg(pg_token))
    }

    async fn get_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<RefreshTokenModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::ConnectionFailed(msg)
            })?;

        let now = Utc::now().naive_utc();

        let pg_token = refresh_tokens::table
            .filter(refresh_tokens::token.eq(token))
            .filter(refresh_tokens::created_at.lt(&now))
            .filter(refresh_tokens::expires_at.gt(&now))
            .filter(refresh_tokens::used.eq(false))
            .first::<PgRefreshToken>(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotFound(msg)
            })?;

        Ok(RefreshTokenMapper::from_pg(pg_token))
    }

    async fn use_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<RefreshTokenModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotUpdated(msg)
            })?;

        let now = Utc::now().naive_utc();

        let pg_token = diesel::update(refresh_tokens::table)
            .filter(refresh_tokens::token.eq(token))
            .filter(refresh_tokens::created_at.lt(&now))
            .filter(refresh_tokens::expires_at.gt(&now))
            .filter(refresh_tokens::used.eq(false))
            .set(refresh_tokens::used.eq(true))
            .get_result::<PgRefreshToken>(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotFound(msg)
            })?;

        Ok(RefreshTokenMapper::from_pg(pg_token))
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

        let affected_rows = diesel::delete(refresh_tokens::table)
            .filter(refresh_tokens::token.eq(token))
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
