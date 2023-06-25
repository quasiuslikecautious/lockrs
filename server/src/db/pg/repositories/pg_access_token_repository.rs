use std::sync::Arc;

use async_trait::async_trait;
use chrono::offset::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    oauth2::{
        mappers::AccessTokenMapper,
        models::{AccessTokenCreateModel, AccessTokenModel},
    },
    pg::{models::PgAccessToken, schema::access_tokens},
    repositories::{AccessTokenRepository, AccessTokenRepositoryError},
    DbContext,
};

pub struct PgAccessTokenRepository {
    db_context: Arc<DbContext>,
}

impl PgAccessTokenRepository {
    pub fn new(db_context: &Arc<DbContext>) -> Self {
        Self {
            db_context: Arc::clone(db_context),
        }
    }
}

#[async_trait]
impl AccessTokenRepository for PgAccessTokenRepository {
    async fn create(
        &self,
        token_create: &AccessTokenCreateModel,
    ) -> Result<AccessTokenModel, AccessTokenRepositoryError> {
        let conn = &mut self
            .db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| AccessTokenRepositoryError::BadConnection)?;

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
            .map_err(|_| AccessTokenRepositoryError::NotCreated)?;

        Ok(AccessTokenMapper::from_pg(pg_token))
    }

    async fn get_by_token(
        &self,
        token: &str,
    ) -> Result<AccessTokenModel, AccessTokenRepositoryError> {
        let conn = &mut self
            .db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| AccessTokenRepositoryError::BadConnection)?;

        let now = Utc::now().naive_utc();

        let pg_token = access_tokens::table
            .filter(access_tokens::token.eq(token))
            .filter(access_tokens::created_at.lt(&now))
            .filter(access_tokens::expires_at.gt(&now))
            .first::<PgAccessToken>(conn)
            .await
            .map_err(|_| AccessTokenRepositoryError::NotFound)?;

        Ok(AccessTokenMapper::from_pg(pg_token))
    }

    async fn delete_by_token(&self, token: &str) -> Result<(), AccessTokenRepositoryError> {
        let conn = &mut self
            .db_context
            .as_ref()
            .get_pg_connection()
            .await
            .map_err(|_| AccessTokenRepositoryError::BadConnection)?;

        let affected_rows = diesel::delete(access_tokens::table)
            .filter(access_tokens::token.eq(token))
            .execute(conn)
            .await
            .map_err(|_| AccessTokenRepositoryError::NotFound)?;

        if affected_rows != 1 {
            return Err(AccessTokenRepositoryError::BadDelete);
        }

        Ok(())
    }
}
