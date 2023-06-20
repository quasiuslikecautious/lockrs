use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::{
        models::{SessionCreateModel, SessionUpdateModel},
        responses::{EndSessionResponse, NewSessionResponse, SessionResponse},
        services::{SessionService, SessionServiceError},
    },
    redis::get_connection_from_pool,
    utils::extractors::{BearerAuth, SessionJwt},
    AppState,
};

pub struct SessionController;

#[derive(Deserialize)]
pub struct SessionUpdateRequest {
    refresh: bool,
}

impl SessionController {
    pub async fn read_all(
        State(_state): State<Arc<AppState>>,
        Path(user_id): Path<Uuid>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/users/{}/sessions", user_id),
        )
    }

    pub async fn create(
        State(state): State<Arc<AppState>>,
        BearerAuth(session_token): BearerAuth,
    ) -> Result<NewSessionResponse, SessionControllerError> {
        let mut redis_connection = get_connection_from_pool(&state.redis_pool)
            .await
            .map_err(|_| SessionControllerError::InternalError)?;

        let session_create = SessionCreateModel { session_token };

        let session = SessionService::create_session(
            redis_connection.as_mut(),
            &session_create,
            &state.config.auth_interval,
        )
        .await
        .map_err(|err| match err {
            SessionServiceError::Token => SessionControllerError::SessionToken,
            _ => SessionControllerError::InternalError,
        })?;

        Ok(NewSessionResponse {
            jwt_util: Arc::clone(&state.jwt_util),
            id: session.id,
            user_id: session.user_id,
            expires_at: session.expires_at,
        })
    }

    pub async fn read(
        State(state): State<Arc<AppState>>,
        SessionJwt(jwt): SessionJwt,
        Path(session_id): Path<String>,
    ) -> Result<SessionResponse, SessionControllerError> {
        if jwt.id != session_id {
            return Err(SessionControllerError::Jwt);
        }

        let mut redis_connection = get_connection_from_pool(&state.redis_pool)
            .await
            .map_err(|_| SessionControllerError::InternalError)?;

        let session =
            SessionService::get_session(redis_connection.as_mut(), &jwt.user_id, &session_id)
                .await
                .map_err(|err| match err {
                    SessionServiceError::NotFound => SessionControllerError::SessionNotFound,
                    _ => SessionControllerError::InternalError,
                })?;

        Ok(SessionResponse {
            id: session.id,
            user_id: session.user_id,
            expires_at: session.expires_at,
        })
    }

    pub async fn update(
        State(state): State<Arc<AppState>>,
        SessionJwt(jwt): SessionJwt,
        Path(session_id): Path<String>,
        Json(session_update_request): Json<SessionUpdateRequest>,
    ) -> Result<SessionResponse, SessionControllerError> {
        if jwt.id != session_id {
            return Err(SessionControllerError::Jwt);
        }

        let mut redis_connection = get_connection_from_pool(&state.redis_pool)
            .await
            .map_err(|_| SessionControllerError::InternalError)?;

        let session_update = SessionUpdateModel {
            refresh: session_update_request.refresh,
        };

        let session = SessionService::update_session(
            redis_connection.as_mut(),
            &jwt.user_id,
            &session_id,
            &session_update,
            &state.config.auth_interval,
        )
        .await
        .map_err(|_| SessionControllerError::InternalError)?;

        Ok(SessionResponse {
            id: session.id,
            user_id: session.user_id,
            expires_at: session.expires_at,
        })
    }

    pub async fn delete(
        State(state): State<Arc<AppState>>,
        SessionJwt(jwt): SessionJwt,
        Path(session_id): Path<String>,
    ) -> Result<EndSessionResponse, SessionControllerError> {
        if jwt.id != session_id {
            return Err(SessionControllerError::Jwt);
        }

        let mut redis_connection = get_connection_from_pool(&state.redis_pool)
            .await
            .map_err(|_| SessionControllerError::InternalError)?;

        SessionService::delete_session(redis_connection.as_mut(), &jwt.user_id)
            .await
            .map_err(|err| match err {
                SessionServiceError::NotFound => SessionControllerError::SessionNotFound,
                _ => SessionControllerError::InternalError,
            })?;

        Ok(EndSessionResponse {})
    }
}

pub enum SessionControllerError {
    InternalError,
    SessionNotFound,
    SessionToken,
    Jwt,
}

impl SessionControllerError {
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::InternalError => {
                "An error has occurred while processing your request. Please try again later."
            }
            Self::SessionNotFound => "Session token not found.",
            Self::SessionToken => "The provided session token is invalid.",
            Self::Jwt => "You do not have permission to access this resource.",
        }
    }
}

impl IntoResponse for SessionControllerError {
    fn into_response(self) -> axum::response::Response {
        println!("response error: {}", self.error_code());
        (StatusCode::BAD_REQUEST, self.error_code()).into_response()
    }
}
