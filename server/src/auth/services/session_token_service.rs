use base64::{engine::general_purpose, Engine as _};
use chrono::{offset::Utc, Duration};
use rand::Rng;
use redis::AsyncCommands;
use uuid::Uuid;

use crate::{auth::models::SessionTokenModel, redis::AsyncRedisConnection};

pub struct SessionTokenService;

impl SessionTokenService {
    pub async fn create_session_token(
        redis_connection: &mut AsyncRedisConnection,
        user_id: &Uuid,
    ) -> Result<SessionTokenModel, SessionTokenServiceError> {
        let token = Self::generate_session_token();
        let key = Self::into_redis_key(token.as_str());

        let ttl = Duration::minutes(5);
        let expires_at = (Utc::now() + ttl).timestamp_millis();

        let token_data = SessionTokenModel {
            token,
            user_id: *user_id,
            expires_at,
        };
        let value = serde_json::to_string(&token_data).unwrap();

        redis::cmd("SET")
            .arg(key.as_str())
            .arg(value.as_str())
            .arg("PXAT")
            .arg(expires_at)
            .query_async(redis_connection)
            .await
            .map_err(|_| SessionTokenServiceError::NotCreated)?;

        Ok(token_data)
    }

    fn generate_session_token() -> String {
        let mut rng = rand::thread_rng();
        let bytes = (0..32).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();

        general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }

    pub async fn validate_session_token(
        redis_connection: &mut AsyncRedisConnection,
        token: &str,
    ) -> Result<SessionTokenModel, SessionTokenServiceError> {
        let key = Self::into_redis_key(token);
        let value: String = redis_connection
            .get(key.as_str())
            .await
            .map_err(|_| SessionTokenServiceError::NotFound)?;

        let token = serde_json::from_str(value.as_str())
            .map_err(|_| SessionTokenServiceError::TokenData)?;

        let deleted: i64 = redis::cmd("DEL")
            .arg(key.as_str())
            .query_async(redis_connection)
            .await
            .map_err(|_| SessionTokenServiceError::StillExists)?;

        if deleted != 1 {
            return Err(SessionTokenServiceError::StillExists);
        }

        Ok(token)
    }

    pub async fn delete_session_token(
        _redis_connection: &mut AsyncRedisConnection,
        _token: &str,
    ) -> Result<SessionTokenModel, SessionTokenServiceError> {
        todo!();
    }

    fn into_redis_key(token: &str) -> String {
        format!("session_token:{}", token)
    }
}

pub enum SessionTokenServiceError {
    NotCreated,
    NotFound,
    TokenData,
    StillExists,
}
