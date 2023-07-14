use std::sync::Arc;

use async_trait::async_trait;
use redis::AsyncCommands;

use crate::{
    auth::models::SessionTokenModel,
    db::{
        repositories::{RepositoryError, SessionTokenRepository},
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
        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let key = Self::into_redis_key(token.token.as_str());
        let value = serde_json::to_string(token).unwrap();

        redis::cmd("SET")
            .arg(key.as_str())
            .arg(value.as_str())
            .arg("PXAT")
            .arg(token.expires_at)
            .query_async(conn)
            .await
            .map_err(|_| RepositoryError::NotCreated)?;

        Ok(token.clone())
    }

    async fn get_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<SessionTokenModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let key = Self::into_redis_key(token);
        let value: String = conn
            .get(key.as_str())
            .await
            .map_err(|_| RepositoryError::NotFound)?;

        serde_json::from_str(value.as_str()).map_err(|_| RepositoryError::BadData)
    }

    async fn delete_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<(), RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let key = Self::into_redis_key(token);

        let deleted: i64 = redis::cmd("DEL")
            .arg(key.as_str())
            .query_async(conn)
            .await
            .map_err(|_| RepositoryError::NotDeleted)?;

        if deleted != 1 {
            return Err(RepositoryError::NotDeleted);
        }

        Ok(())
    }
}
