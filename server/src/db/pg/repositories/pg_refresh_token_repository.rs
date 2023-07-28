use std::sync::Arc;

use async_trait::async_trait;
use chrono::offset::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    db::{
        pg::{models::PgRefreshToken, schema::refresh_tokens},
        repositories::{QueryFailure, RefreshTokenRepository, RepositoryError},
        DbContext,
    },
    oauth2::v1::{
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
        tracing::trace!(
            method = "create",
        );

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

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
            .map_err(RepositoryError::map_diesel_create)?;

        Ok(RefreshTokenMapper::from_pg(pg_token))
    }

    async fn get_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<RefreshTokenModel, RepositoryError> {
        tracing::trace!(
            method = "get_by_token",
        );

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let now = Utc::now().naive_utc();

        let pg_token = refresh_tokens::table
            .filter(refresh_tokens::token.eq(token))
            .filter(refresh_tokens::created_at.lt(&now))
            .filter(refresh_tokens::expires_at.gt(&now))
            .filter(refresh_tokens::used.eq(false))
            .first::<PgRefreshToken>(conn)
            .await
            .map_err(RepositoryError::map_diesel_found)?;

        Ok(RefreshTokenMapper::from_pg(pg_token))
    }

    async fn use_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<RefreshTokenModel, RepositoryError> {
        tracing::trace!(
            method = "use_by_token",
        );

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let now = Utc::now().naive_utc();

        let pg_token = diesel::update(refresh_tokens::table)
            .filter(refresh_tokens::token.eq(token))
            .filter(refresh_tokens::created_at.lt(&now))
            .filter(refresh_tokens::expires_at.gt(&now))
            .filter(refresh_tokens::used.eq(false))
            .set(refresh_tokens::used.eq(true))
            .get_result::<PgRefreshToken>(conn)
            .await
            .map_err(RepositoryError::map_diesel_update)?;

        Ok(RefreshTokenMapper::from_pg(pg_token))
    }

    async fn delete_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<(), RepositoryError> {
        tracing::trace!(
            method = "delete_by_token",
        );

        let conn = &mut db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(RepositoryError::from)?;

        let affected_rows = diesel::delete(refresh_tokens::table)
            .filter(refresh_tokens::token.eq(token))
            .execute(conn)
            .await
            .map_err(RepositoryError::map_diesel_delete)?;

        if affected_rows != 1 {
            let msg = format!(
                "Expected 1 row to be affected by delete, but found {}",
                affected_rows
            );

            tracing::error!(error = msg);
            return Err(RepositoryError::QueryFailed(msg, QueryFailure::NotDeleted));
        }

        Ok(())
    }
}
