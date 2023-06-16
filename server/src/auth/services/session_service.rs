use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use rand::Rng;
use redis::AsyncCommands;
use uuid::Uuid;

use crate::{
    auth::models::{SessionCreateModel, SessionModel, SessionUpdateModel},
    redis::AsyncRedisConnection,
};

use super::SessionTokenService;

pub struct SessionService;

impl SessionService {
    pub async fn create_session(
        redis_connection: &mut AsyncRedisConnection,
        token: &SessionCreateModel,
        session_duration: &Duration,
    ) -> Result<SessionModel, SessionServiceError> {
        let token = SessionTokenService::validate_session_token(
            redis_connection,
            token.session_token.as_str(),
        )
        .await
        .map_err(|_| SessionServiceError::Token)?;

        let session_id = Self::generate_session_id();
        let key = Self::into_session_key(session_id.as_str());

        let expires_at = (Utc::now() + *session_duration).timestamp_millis();

        let session_data = SessionModel::new(&session_id, &token.user_id, &expires_at);
        let value = serde_json::to_string(&session_data).unwrap();

        redis::cmd("SET")
            .arg(key.as_str())
            .arg(value.as_str())
            .arg("PXAT")
            .arg(expires_at)
            .query_async(redis_connection)
            .await
            .map_err(|_| SessionServiceError::NotCreated)?;

        Ok(session_data)
    }

    pub async fn get_session_by_id(
        redis_connection: &mut AsyncRedisConnection,
        id: &str,
    ) -> Result<SessionModel, SessionServiceError> {
        let key = Self::into_session_key(id);
        let value: String = redis_connection
            .get(key.as_str())
            .await
            .map_err(|_| SessionServiceError::NotFound)?;

        let session_data =
            serde_json::from_str(&value).map_err(|_| SessionServiceError::SessionData)?;

        Ok(session_data)
    }

    pub async fn get_sessions_by_user_id(
        _redis_connection: &mut AsyncRedisConnection,
        _user_id: &Uuid,
    ) -> Result<SessionModel, SessionServiceError> {
        todo!();
    }

    pub async fn update_session_by_id(
        redis_connection: &mut AsyncRedisConnection,
        id: &str,
        update_model: &SessionUpdateModel,
        session_duration: &Duration,
    ) -> Result<SessionModel, SessionServiceError> {
        let mut session = Self::get_session_by_id(redis_connection, id).await?;

        if !update_model.refresh {
            return Ok(session);
        }

        let key = Self::into_session_key(id);

        let expires_at = (Utc::now() + *session_duration).timestamp_millis();
        session.expires_at = expires_at;

        let value = serde_json::to_string(&session).unwrap();

        redis::cmd("SET")
            .arg(key.as_str())
            .arg(value.as_str())
            .arg("PXAT")
            .arg(expires_at)
            .query_async(redis_connection)
            .await
            .map_err(|_| SessionServiceError::NotCreated)?;

        Ok(session)
    }

    pub async fn delete_session(
        redis_connection: &mut AsyncRedisConnection,
        id: &str,
    ) -> Result<(), SessionServiceError> {
        let key = Self::into_session_key(id);
        redis_connection
            .del(key.as_str())
            .await
            .map_err(|_| SessionServiceError::NotDeleted)?;

        Ok(())
    }

    fn into_session_key(session_id: &str) -> String {
        format!("session:{}", session_id)
    }

    fn generate_session_id() -> String {
        let mut rng = rand::thread_rng();
        let bytes = (0..32).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();

        general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }
}

pub enum SessionServiceError {
    NotCreated,
    NotFound,
    NotUpdated,
    NotDeleted,
    SessionData,
    Token,
}