use std::sync::Arc;

use async_trait::async_trait;
use redis::AsyncCommands;

use crate::{
    api::v1::models::SessionTokenModel,
    db::{
        repositories::{QueryFailure, RepositoryError, SessionTokenRepository},
        DbContext,
    },
};

pub struct RedisSessionTokenRepository;

impl RedisSessionTokenRepository {
    fn into_redis_key(token: &str) -> String {
        format!("session_token:{}", token)
    }
}

#[async_trait]
impl SessionTokenRepository for RedisSessionTokenRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        token: &SessionTokenModel,
    ) -> Result<SessionTokenModel, RepositoryError> {
        tracing::trace!(method = "create");

        let key = Self::into_redis_key(token.token.as_str());
        let value = serde_json::to_string(token).unwrap();

        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(RepositoryError::from)?;

        redis::cmd("SET")
            .arg(key.as_str())
            .arg(value.as_str())
            .arg("PXAT")
            .arg(token.expires_at)
            .query_async(conn)
            .await
            .map_err(RepositoryError::map_redis_create)?;

        Ok(token.clone())
    }

    async fn get_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<SessionTokenModel, RepositoryError> {
        tracing::trace!(method = "get_by_token");

        let key = Self::into_redis_key(token);

        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(RepositoryError::from)?;

        let value: String = conn
            .get(key.as_str())
            .await
            .map_err(RepositoryError::map_redis)?;

        serde_json::from_str(value.as_str()).map_err(|_| {
            let msg = format!(
                "Invalid JSON data format for data stored at token {}",
                token
            );

            tracing::error!(error = msg);

            RepositoryError::InternalError
        })
    }

    async fn delete_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<(), RepositoryError> {
        tracing::trace!(method = "delete_by_token");

        let key = Self::into_redis_key(token);

        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(RepositoryError::from)?;

        let deleted: i64 = redis::cmd("DEL")
            .arg(key.as_str())
            .query_async(conn)
            .await
            .map_err(RepositoryError::map_redis)?;

        if deleted != 1 {
            let msg = format!(
                "Expected 1 row to be affected by delete, but found {}",
                deleted
            );

            tracing::error!(error = msg);
            return Err(RepositoryError::QueryFailed(QueryFailure::NotDeleted));
        }

        Ok(())
    }
}
