use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use rand::Rng;
use uuid::Uuid;

use crate::{
    auth::models::{SessionCreateModel, SessionModel, SessionUpdateModel},
    db::{
        repositories::{SessionRepository, SessionTokenRepository},
        DbContext,
    },
};

use super::SessionTokenService;

pub struct SessionService;

impl SessionService {
    pub async fn create_session(
        db_context: &Arc<DbContext>,
        session_repository: &dyn SessionRepository,
        session_token_repository: &dyn SessionTokenRepository,
        token: &SessionCreateModel,
        session_duration: &Duration,
    ) -> Result<SessionModel, SessionServiceError> {
        let token = SessionTokenService::validate_session_token(
            db_context,
            session_token_repository,
            token.session_token.as_str(),
        )
        .await
        .map_err(|_| SessionServiceError::Token)?;

        let user_id = token.user_id;
        let session_id = Self::generate_session_id();
        let expires_at = (Utc::now() + *session_duration).timestamp_millis();

        let session_data = SessionModel::new(&session_id, &user_id, &expires_at);
        session_repository
            .create(db_context, &session_data)
            .await
            .map_err(|_| SessionServiceError::NotCreated)
    }

    pub async fn get_session(
        db_context: &Arc<DbContext>,
        session_repository: &dyn SessionRepository,
        user_id: &Uuid,
        session_id: &str,
    ) -> Result<SessionModel, SessionServiceError> {
        session_repository
            .get_by_hash(db_context, session_id, user_id)
            .await
            .map_err(|_| SessionServiceError::NotFound)
    }

    pub async fn update_session(
        db_context: &Arc<DbContext>,
        session_repository: &dyn SessionRepository,
        user_id: &Uuid,
        session_id: &str,
        update_model: &SessionUpdateModel,
        session_duration: &Duration,
    ) -> Result<SessionModel, SessionServiceError> {
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
            .map_err(|_| SessionServiceError::NotUpdated)
    }

    pub async fn delete_session(
        db_context: &Arc<DbContext>,
        session_repository: &dyn SessionRepository,
        user_id: &Uuid,
    ) -> Result<(), SessionServiceError> {
        session_repository
            .delete_by_user_id(db_context, user_id)
            .await
            .map_err(|_| SessionServiceError::BadDelete)
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
    BadDelete,
    Token,
}
