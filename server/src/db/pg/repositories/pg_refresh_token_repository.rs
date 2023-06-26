use std::sync::Arc;

use async_trait::async_trait;
use chrono::offset::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    oauth2::{
        mappers::RefreshTokenMapper,
        models::{RefreshTokenCreateModel, RefreshTokenModel},
    },
    pg::{models::PgRefreshToken, schema::refresh_tokens},
    repositories::{RefreshTokenRepository, RefreshTokenRepositoryError},
    DbContext,
};

pub struct PgRefreshTokenRepository {
    db_context: Arc<DbContext>,
}

impl PgRefreshTokenRepository {
    pub fn new(db_context: &Arc<DbContext>) -> Self {
        Self {
            db_context: Arc::clone(db_context),
        }
    }
}

#[async_trait]
impl RefreshTokenRepository for PgRefreshTokenRepository {
    async fn create(
        &self,
        token_create: &RefreshTokenCreateModel,
    ) -> Result<RefreshTokenModel, RefreshTokenRepositoryError> {
        let conn = &mut self
            .db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RefreshTokenRepositoryError::BadConnection)?;

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
            .map_err(|_| RefreshTokenRepositoryError::NotCreated)?;

        Ok(RefreshTokenMapper::from_pg(pg_token))
    }

    async fn get_by_token(
        &self,
        token: &str,
    ) -> Result<RefreshTokenModel, RefreshTokenRepositoryError> {
        let conn = &mut self
            .db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RefreshTokenRepositoryError::BadConnection)?;

        let now = Utc::now().naive_utc();

        let pg_token = refresh_tokens::table
            .filter(refresh_tokens::token.eq(token))
            .filter(refresh_tokens::created_at.lt(&now))
            .filter(refresh_tokens::expires_at.gt(&now))
            .filter(refresh_tokens::used.eq(false))
            .first::<PgRefreshToken>(conn)
            .await
            .map_err(|_| RefreshTokenRepositoryError::NotFound)?;

        Ok(RefreshTokenMapper::from_pg(pg_token))
    }

    async fn use_by_token(
        &self,
        token: &str,
    ) -> Result<RefreshTokenModel, RefreshTokenRepositoryError> {
        let conn = &mut self
            .db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RefreshTokenRepositoryError::NotUpdated)?;

        let now = Utc::now().naive_utc();

        let pg_token = diesel::update(refresh_tokens::table)
            .filter(refresh_tokens::token.eq(token))
            .filter(refresh_tokens::created_at.lt(&now))
            .filter(refresh_tokens::expires_at.gt(&now))
            .filter(refresh_tokens::used.eq(false))
            .set(refresh_tokens::used.eq(true))
            .get_result::<PgRefreshToken>(conn)
            .await
            .map_err(|_| RefreshTokenRepositoryError::NotFound)?;

        Ok(RefreshTokenMapper::from_pg(pg_token))
    }

    async fn delete_by_token(&self, token: &str) -> Result<(), RefreshTokenRepositoryError> {
        let conn = &mut self
            .db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| RefreshTokenRepositoryError::BadConnection)?;

        let affected_rows = diesel::delete(refresh_tokens::table)
            .filter(refresh_tokens::token.eq(token))
            .execute(conn)
            .await
            .map_err(|_| RefreshTokenRepositoryError::NotFound)?;

        if affected_rows != 1 {
            return Err(RefreshTokenRepositoryError::BadDelete);
        }

        Ok(())
    }
}
