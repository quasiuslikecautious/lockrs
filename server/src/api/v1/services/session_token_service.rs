use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};
use chrono::{offset::Utc, Duration};
use rand::Rng;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    api::v1::models::SessionTokenModel,
    db::{
        repositories::{QueryFailure, RepositoryError, SessionTokenRepository},
        DbContext,
    },
};

pub struct SessionTokenService;

impl SessionTokenService {
    pub async fn create_session_token(
        db_context: &Arc<DbContext>,
        session_token_repository: &dyn SessionTokenRepository,
        user_id: &Uuid,
    ) -> Result<SessionTokenModel, SessionTokenServiceError> {
        tracing::trace!(method = "create_session_token", ?user_id);

        let ttl = Duration::minutes(5);
        let expires_at = (Utc::now() + ttl).timestamp_millis();

        let token_data = SessionTokenModel {
            token: Self::generate_session_token(),
            user_id: *user_id,
            expires_at,
        };

        session_token_repository
            .create(db_context, &token_data)
            .await
            .map_err(SessionTokenServiceError::from)
    }

    fn generate_session_token() -> String {
        let mut rng = rand::thread_rng();
        let bytes = (0..32).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();

        general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }

    pub async fn validate_session_token(
        db_context: &Arc<DbContext>,
        session_token_repository: &dyn SessionTokenRepository,
        token: &str,
    ) -> Result<SessionTokenModel, SessionTokenServiceError> {
        tracing::trace!(method = "validate_session_token",);

        let token = session_token_repository
            .get_by_token(db_context, token)
            .await
            .map_err(SessionTokenServiceError::from)?;

        Self::delete_session_token(db_context, session_token_repository, token.token.as_str())
            .await?;

        Ok(token)
    }

    pub async fn delete_session_token(
        db_context: &Arc<DbContext>,
        session_token_repository: &dyn SessionTokenRepository,
        token: &str,
    ) -> Result<(), SessionTokenServiceError> {
        tracing::trace!(method = "delete_session_token",);

        session_token_repository
            .delete_by_token(db_context, token)
            .await
            .map_err(SessionTokenServiceError::from)
    }
}

#[derive(Debug, Error)]
pub enum SessionTokenServiceError {
    #[error("SESSION TOKEN SERVICE ERROR :: Not Created")]
    NotCreated,
    #[error("SESSION TOKEN SERVICE ERROR :: Not Found")]
    NotFound,
    #[error("SESSION TOKEN SERVICE ERROR :: Not Deleted")]
    NotDeleted,

    #[error("SESSION TOKEN SERVICE ERROR :: Internal Error")]
    InternalError,
}

impl From<RepositoryError> for SessionTokenServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            RepositoryError::QueryFailed(_, query_err) => match query_err {
                QueryFailure::NotCreated => Self::NotCreated,
                QueryFailure::NotFound => Self::NotFound,
                QueryFailure::NotDeleted => Self::NotDeleted,

                _ => Self::InternalError,
            },
            RepositoryError::InternalError(_) => Self::InternalError,
        }
    }
}
