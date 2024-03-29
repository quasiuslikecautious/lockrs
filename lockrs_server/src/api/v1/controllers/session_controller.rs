use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::{
    api::v1::{
        models::{SessionCreateModel, SessionUpdateModel},
        responses::{EndSessionResponse, NewSessionResponse, SessionResponse},
        services::{SessionService, SessionServiceError},
    },
    utils::extractors::{BearerAuth, SessionJwt},
    AppState,
};

pub struct SessionController;

#[derive(Debug, Deserialize)]
pub struct SessionUpdateRequest {
    refresh: bool,
}

impl SessionController {
    pub async fn read_active(
        State(state): State<AppState>,
        SessionJwt(jwt): SessionJwt,
    ) -> Result<SessionResponse, SessionControllerError> {
        tracing::trace!(method = "read_current", session_id = jwt.id);

        let db_context = &state.db_context;
        let session_repository = &*state.repository_container.as_ref().session_repository;

        let session = SessionService::get_session(
            db_context,
            session_repository,
            &jwt.user_id,
            jwt.id.as_str(),
        )
        .await
        .map_err(SessionControllerError::from)?;

        Ok(SessionResponse {
            id: session.id,
            user_id: session.user_id,
            expires_at: session.expires_at,
        })
    }

    pub async fn create(
        State(state): State<AppState>,
        BearerAuth(session_token): BearerAuth,
    ) -> Result<NewSessionResponse, SessionControllerError> {
        tracing::trace!(method = "create");

        let db_context = &state.db_context;
        let session_repository = &*state.repository_container.as_ref().session_repository;
        let session_token_repository =
            &*state.repository_container.as_ref().session_token_repository;

        let session_create = SessionCreateModel::new(session_token.as_str());

        let session = SessionService::create_session(
            db_context,
            session_repository,
            session_token_repository,
            &session_create,
            &state.config.auth_interval,
        )
        .await
        .map_err(SessionServiceError::from)?;

        Ok(NewSessionResponse {
            jwt_util: Arc::clone(&state.jwt_util),
            id: session.id,
            user_id: session.user_id,
            expires_at: session.expires_at,
        })
    }

    pub async fn read(
        State(state): State<AppState>,
        SessionJwt(jwt): SessionJwt,
        Path(session_id): Path<String>,
    ) -> Result<SessionResponse, SessionControllerError> {
        tracing::trace!(method = "read", session_id = session_id);

        let db_context = &state.db_context;
        let session_repository = &*state.repository_container.as_ref().session_repository;

        let session =
            SessionService::get_session(db_context, session_repository, &jwt.user_id, &session_id)
                .await
                .map_err(SessionControllerError::from)?;

        Ok(SessionResponse {
            id: session.id,
            user_id: session.user_id,
            expires_at: session.expires_at,
        })
    }

    pub async fn update(
        State(state): State<AppState>,
        SessionJwt(jwt): SessionJwt,
        Path(session_id): Path<String>,
        Json(session_update_request): Json<SessionUpdateRequest>,
    ) -> Result<SessionResponse, SessionControllerError> {
        tracing::trace!(
            method = "update",
            session_id = session_id,
            params = ?session_update_request
        );

        let db_context = &state.db_context;
        let session_repository = &*state.repository_container.as_ref().session_repository;

        let session_update = SessionUpdateModel::new(session_update_request.refresh);

        let session = SessionService::update_session(
            db_context,
            session_repository,
            &jwt.user_id,
            &session_id,
            &session_update,
            &state.config.auth_interval,
        )
        .await
        .map_err(SessionControllerError::from)?;

        Ok(SessionResponse {
            id: session.id,
            user_id: session.user_id,
            expires_at: session.expires_at,
        })
    }

    pub async fn delete(
        State(state): State<AppState>,
        SessionJwt(jwt): SessionJwt,
        Path(session_id): Path<String>,
    ) -> Result<EndSessionResponse, SessionControllerError> {
        tracing::trace!(method = "delete", session_id = session_id);

        if jwt.id != session_id {
            return Err(SessionControllerError::Jwt);
        }

        let db_context = &state.db_context;
        let session_repository = &*state.repository_container.as_ref().session_repository;

        SessionService::delete_session(db_context, session_repository, &jwt.user_id)
            .await
            .map_err(SessionControllerError::from)?;

        Ok(EndSessionResponse {})
    }
}

pub enum SessionControllerError {
    Jwt,
    SessionToken,
    NotFound,
    BadRequest,
    InternalError,
}

impl SessionControllerError {
    pub fn error_code(&self) -> StatusCode {
        match self {
            Self::Jwt => StatusCode::UNAUTHORIZED,
            Self::SessionToken => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_message(&self) -> &'static str {
        match self {
            Self::Jwt => "You do not have permission to access this resource.",
            Self::SessionToken => "The provided session token is invalid.",
            Self::NotFound => "Session token not found.",
            Self::BadRequest => "Unable to perform the requested operation.",
            Self::InternalError => {
                "An error has occurred while processing your request. Please try again later."
            }
        }
    }
}

impl From<SessionServiceError> for SessionControllerError {
    fn from(err: SessionServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            SessionServiceError::Token => Self::SessionToken,
            SessionServiceError::NotFound => Self::NotFound,

            SessionServiceError::NotCreated => Self::BadRequest,
            SessionServiceError::NotUpdated => Self::BadRequest,
            SessionServiceError::NotDeleted => Self::BadRequest,

            SessionServiceError::InternalError => Self::InternalError,
        }
    }
}

impl IntoResponse for SessionControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
