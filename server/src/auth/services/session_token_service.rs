use base64::{engine::general_purpose, Engine as _};
use chrono::{offset::Utc, Duration};
use rand::Rng;
use uuid::Uuid;

use crate::{auth::models::SessionTokenModel, repositories::SessionTokenRepository};

pub struct SessionTokenService;

impl SessionTokenService {
    pub async fn create_session_token(
        session_token_repository: &Box<dyn SessionTokenRepository>,
        user_id: &Uuid,
    ) -> Result<SessionTokenModel, SessionTokenServiceError> {
        let ttl = Duration::minutes(5);
        let expires_at = (Utc::now() + ttl).timestamp_millis();

        let token_data = SessionTokenModel {
            token: Self::generate_session_token(),
            user_id: *user_id,
            expires_at,
        };

        session_token_repository
            .create(&token_data)
            .await
            .map_err(|_| SessionTokenServiceError::NotCreated)
    }

    fn generate_session_token() -> String {
        let mut rng = rand::thread_rng();
        let bytes = (0..32).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();

        general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }

    pub async fn validate_session_token(
        session_token_repository: &Box<dyn SessionTokenRepository>,
        token: &str,
    ) -> Result<SessionTokenModel, SessionTokenServiceError> {
        let token = session_token_repository
            .get_by_token(token)
            .await
            .map_err(|_| SessionTokenServiceError::NotFound)?;

        Self::delete_session_token(session_token_repository, token.token.as_str()).await?;

        Ok(token)
    }

    pub async fn delete_session_token(
        session_token_repository: &Box<dyn SessionTokenRepository>,
        token: &str,
    ) -> Result<(), SessionTokenServiceError> {
        session_token_repository
            .delete_by_token(token)
            .await
            .map_err(|_| SessionTokenServiceError::BadDelete)
    }
}

pub enum SessionTokenServiceError {
    NotCreated,
    NotFound,
    TokenData,
    BadDelete,
}
