use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    auth::models::SessionModel,
    repositories::{SessionRepository, SessionRepositoryError},
    DbContext,
};

pub struct RedisSessionRepository {
    db_context: Arc<DbContext>,
}

impl RedisSessionRepository {
    pub fn new(db_context: &Arc<DbContext>) -> Self {
        Self {
            db_context: Arc::clone(db_context),
        }
    }

    fn into_session_key(session_id: &str) -> String {
        format!("active_session:{}", session_id)
    }

    fn into_user_key(user_id: &Uuid) -> String {
        format!("user:{}", user_id)
    }
}

#[async_trait]
impl SessionRepository for RedisSessionRepository {
    async fn create(&self, session: &SessionModel) -> Result<SessionModel, SessionRepositoryError> {
        let conn = &mut self
            .db_context
            .get_redis_connection()
            .await
            .map_err(|_| SessionRepositoryError::BadConnection)?;

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
            .map_err(|_| SessionRepositoryError::NotCreated)?;

        Ok(session.clone())
    }

    async fn get_by_hash(
        &self,
        session_id: &str,
        user_id: &Uuid,
    ) -> Result<SessionModel, SessionRepositoryError> {
        let conn = &mut self
            .db_context
            .get_redis_connection()
            .await
            .map_err(|_| SessionRepositoryError::BadConnection)?;

        let user_key = Self::into_user_key(user_id);
        let session_key = Self::into_session_key(session_id);

        let value: String = redis::cmd("HGET")
            .arg(user_key.as_str())
            .arg(session_key.as_str())
            .query_async(conn)
            .await
            .map_err(|_| SessionRepositoryError::NotFound)?;

        serde_json::from_str(value.as_str()).map_err(|_| SessionRepositoryError::BadData)
    }

    async fn update(&self, session: &SessionModel) -> Result<SessionModel, SessionRepositoryError> {
        let conn = &mut self
            .db_context
            .get_redis_connection()
            .await
            .map_err(|_| SessionRepositoryError::BadConnection)?;

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
            .map_err(|_| SessionRepositoryError::NotUpdated)?;

        Ok(session.clone())
    }

    async fn delete_by_user_id(&self, id: &Uuid) -> Result<(), SessionRepositoryError> {
        let conn = &mut self
            .db_context
            .get_redis_connection()
            .await
            .map_err(|_| SessionRepositoryError::BadConnection)?;

        let key = Self::into_user_key(id);

        redis::cmd("DEL")
            .arg(key.as_str())
            .query_async(conn)
            .await
            .map_err(|_| SessionRepositoryError::BadDelete)?;

        Ok(())
    }
}
