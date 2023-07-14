use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    auth::models::SessionModel,
    db::{
        repositories::{RepositoryError, SessionRepository},
        DbContext,
    },
};

pub struct RedisSessionRepository;

impl RedisSessionRepository {
    fn into_session_key(session_id: &str) -> String {
        format!("active_session:{}", session_id)
    }

    fn into_user_key(user_id: &Uuid) -> String {
        format!("user:{}", user_id)
    }
}

#[async_trait]
impl SessionRepository for RedisSessionRepository {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        session: &SessionModel,
    ) -> Result<SessionModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let user_id = &session.user_id;
        let user_key = Self::into_user_key(user_id);

        let session_id = &session.id;
        let session_key = Self::into_session_key(session_id.as_str());

        let expires_at = session.expires_at;

        let value = serde_json::to_string(session).unwrap();

        redis::cmd("HSET")
            .arg(user_key.as_str())
            .arg(session_key.as_str())
            .arg(value.as_str())
            .arg("PXAT")
            .arg(expires_at)
            .query_async(conn)
            .await
            .map_err(|_| RepositoryError::NotCreated)?;

        Ok(session.clone())
    }

    async fn get_by_hash(
        &self,
        db_context: &Arc<DbContext>,
        session_id: &str,
        user_id: &Uuid,
    ) -> Result<SessionModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let user_key = Self::into_user_key(user_id);
        let session_key = Self::into_session_key(session_id);

        let value: String = redis::cmd("HGET")
            .arg(user_key.as_str())
            .arg(session_key.as_str())
            .query_async(conn)
            .await
            .map_err(|_| RepositoryError::NotFound)?;

        serde_json::from_str(value.as_str()).map_err(|_| RepositoryError::BadData)
    }

    async fn update(
        &self,
        db_context: &Arc<DbContext>,
        session: &SessionModel,
    ) -> Result<SessionModel, RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let user_key = Self::into_user_key(&session.user_id);
        let session_key = Self::into_session_key(session.id.as_str());
        let expires_at = &session.expires_at;

        let value = serde_json::to_string(&session).unwrap();

        redis::cmd("HSET")
            .arg(user_key.as_str())
            .arg(session_key.as_str())
            .arg(value.as_str())
            .arg("PXAT")
            .arg(expires_at)
            .query_async(conn)
            .await
            .map_err(|_| RepositoryError::NotUpdated)?;

        Ok(session.clone())
    }

    async fn delete_by_user_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<(), RepositoryError> {
        let conn = &mut db_context
            .as_ref()
            .get_redis_connection()
            .await
            .map_err(|_| RepositoryError::ConnectionFailed)?;

        let key = Self::into_user_key(id);

        redis::cmd("DEL")
            .arg(key.as_str())
            .query_async(conn)
            .await
            .map_err(|_| RepositoryError::NotDeleted)?;

        Ok(())
    }
}
