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
        let key = Self::into_redis_key(token.token.as_str());
        let value = serde_json::to_string(token).unwrap();

        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::Connection(msg)
            })?;

        redis::cmd("SET")
            .arg(key.as_str())
            .arg(value.as_str())
            .arg("PXAT")
            .arg(token.expires_at)
            .query_async(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotCreated(msg)
            })?;

        Ok(token.clone())
    }

    async fn get_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<SessionTokenModel, RepositoryError> {
        let key = Self::into_redis_key(token);

        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::Connection(msg)
            })?;

        let value: String = conn.get(key.as_str()).await.map_err(|err| {
            let msg = format!("{}", err);
            RepositoryError::NotFound(msg)
        })?;

        serde_json::from_str(value.as_str()).map_err(|_| {
            let msg = format!(
                "Invalid JSON data format for data stored at token {}",
                token
            );
            RepositoryError::Database(msg)
        })
    }

    async fn delete_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<(), RepositoryError> {
        let key = Self::into_redis_key(token);

        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::Connection(msg)
            })?;

        let deleted: i64 = redis::cmd("DEL")
            .arg(key.as_str())
            .query_async(conn)
            .await
            .map_err(|err| {
                let msg = format!("{}", err);
                RepositoryError::NotDeleted(msg)
            })?;

        if deleted != 1 {
            let msg = format!(
                "Expected 1 row to be affected by delete, but found {}",
                deleted
            );
            return Err(RepositoryError::NotDeleted(msg));
        }

        Ok(())
    }
}
