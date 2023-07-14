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
            .map_err(|_| RepositoryError::ConnectionFailed)?;

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
            .map_err(|_| RepositoryError::NotCreated)?;

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
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let now = Utc::now().naive_utc();

        let pg_token = refresh_tokens::table
            .filter(refresh_tokens::token.eq(token))
            .filter(refresh_tokens::created_at.lt(&now))
            .filter(refresh_tokens::expires_at.gt(&now))
            .filter(refresh_tokens::used.eq(false))
            .first::<PgRefreshToken>(conn)
            .await
            .map_err(|_| RepositoryError::NotFound)?;

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
            .map_err(|_| RepositoryError::NotUpdated)?;

        let now = Utc::now().naive_utc();

        let pg_token = diesel::update(refresh_tokens::table)
            .filter(refresh_tokens::token.eq(token))
            .filter(refresh_tokens::created_at.lt(&now))
            .filter(refresh_tokens::expires_at.gt(&now))
            .filter(refresh_tokens::used.eq(false))
            .set(refresh_tokens::used.eq(true))
            .get_result::<PgRefreshToken>(conn)
            .await
            .map_err(|_| RepositoryError::NotFound)?;

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
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let affected_rows = diesel::delete(refresh_tokens::table)
            .filter(refresh_tokens::token.eq(token))
            .execute(conn)
            .await
            .map_err(|_| RepositoryError::NotFound)?;

        if affected_rows != 1 {
            return Err(RepositoryError::NotDeleted);
        }

        Ok(())
    }
}
