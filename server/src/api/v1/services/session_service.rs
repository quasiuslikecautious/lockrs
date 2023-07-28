use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use rand::Rng;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    api::v1::{
        models::{SessionCreateModel, SessionModel, SessionUpdateModel},
        services::{SessionTokenService, SessionTokenServiceError},
    },
    db::{
        repositories::{QueryFailure, RepositoryError, SessionRepository, SessionTokenRepository},
        DbContext,
    },
};

pub struct SessionService;

impl SessionService {
    pub async fn create_session(
        db_context: &Arc<DbContext>,
        session_repository: &dyn SessionRepository,
        session_token_repository: &dyn SessionTokenRepository,
        token: &SessionCreateModel,
        session_duration: &Duration,
    ) -> Result<SessionModel, SessionServiceError> {
        tracing::trace!(
            method = "create_session",
            session_duration = session_duration.num_milliseconds()
        );

        let token = SessionTokenService::validate_session_token(
            db_context,
            session_token_repository,
            token.session_token.as_str(),
        )
        .await
        .map_err(SessionServiceError::from)?;

        let user_id = token.user_id;
        let session_id = Self::generate_session_id();
        let expires_at = (Utc::now() + *session_duration).timestamp_millis();

        let session_data = SessionModel::new(&session_id, &user_id, &expires_at);
        session_repository
            .create(db_context, &session_data)
            .await
            .map_err(SessionServiceError::from)
    }

    pub async fn get_session(
        db_context: &Arc<DbContext>,
        session_repository: &dyn SessionRepository,
        user_id: &Uuid,
        session_id: &str,
    ) -> Result<SessionModel, SessionServiceError> {
        tracing::trace!(
            method = "get_session",
            ?user_id,
            session_id
        );

        session_repository
            .get_by_hash(db_context, session_id, user_id)
            .await
            .map_err(SessionServiceError::from)
    }

    pub async fn update_session(
        db_context: &Arc<DbContext>,
        session_repository: &dyn SessionRepository,
        user_id: &Uuid,
        session_id: &str,
        update_model: &SessionUpdateModel,
        session_duration: &Duration,
    ) -> Result<SessionModel, SessionServiceError> {
        tracing::trace!(
            method = "update_session",
            ?user_id,
            session_id,
            session = ?update_model,
            duration = session_duration.num_milliseconds()
        );

        let mut session =
            Self::get_session(db_context, session_repository, user_id, session_id).await?;

        if !update_model.refresh {
            return Ok(session);
        }

        let expires_at = (Utc::now() + *session_duration).timestamp_millis();
        session.expires_at = expires_at;

        session_repository
            .update(db_context, &session)
            .await
            .map_err(SessionServiceError::from)
    }

    pub async fn delete_session(
        db_context: &Arc<DbContext>,
        session_repository: &dyn SessionRepository,
        user_id: &Uuid,
    ) -> Result<(), SessionServiceError> {
        tracing::trace!(
            method = "delete_session",
            ?user_id,
        );

        session_repository
            .delete_by_user_id(db_context, user_id)
            .await
            .map_err(SessionServiceError::from)
    }

    fn generate_session_id() -> String {
        let mut rng = rand::thread_rng();
        let bytes = (0..32).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();

        general_purpose::URL_SAFE_NO_PAD.encode(bytes)
    }
}

#[derive(Debug, Error)]
pub enum SessionServiceError {
    #[error("SESSION SERVICE ERROR :: Not Created :: {0}")]
    NotCreated(String),
    #[error("SESSION SERVICE ERROR :: Not Found :: {0}")]
    NotFound(String),
    #[error("SESSION SERVICE ERROR :: Not Updated :: {0}")]
    NotUpdated(String),
    #[error("SESSION SERVICE ERROR :: Not Deleted :: {0}")]
    NotDeleted(String),
    #[error("SESSION SERVICE ERROR :: Bad Token :: {0}")]
    Token(String),

    #[error("SESSION SERVICE ERROR :: Internal Error :: {0}")]
    InternalError(String),
}

impl From<RepositoryError> for SessionServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            RepositoryError::QueryFailed(msg, query_err) => match query_err {
                QueryFailure::NotCreated => Self::NotCreated(msg),
                QueryFailure::NotFound => Self::NotFound(msg),
                QueryFailure::NotUpdated => Self::NotUpdated(msg),
                QueryFailure::NotDeleted => Self::NotDeleted(msg),

                QueryFailure::AlreadyExists => Self::InternalError(msg),
            },

            RepositoryError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}

impl From<SessionTokenServiceError> for SessionServiceError {
    fn from(err: SessionTokenServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            SessionTokenServiceError::NotFound(msg) => Self::Token(msg),

            SessionTokenServiceError::NotCreated(msg) => Self::InternalError(msg),
            SessionTokenServiceError::NotDeleted(msg) => Self::InternalError(msg),
            SessionTokenServiceError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}
