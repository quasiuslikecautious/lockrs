use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::{
        responses::SessionResponse,
        services::{UserAuthService, UserAuthServiceError},
    },
    db::get_connection_from_pool,
    models::UserAuthModel,
    AppState,
};

pub struct SessionController;

#[derive(Deserialize)]
pub struct SessionCreateRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SessionUpdateRequest {
    token: Option<String>,
}

impl SessionController {
    pub async fn read_all(
        Extension(_state): Extension<Arc<AppState>>,
        Path(user_id): Path<Uuid>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/users/{}/sessions", user_id),
        )
    }

    pub async fn create(
        Extension(state): Extension<Arc<AppState>>,
        Path(_user_id): Path<Uuid>,
        Json(new_session): Json<SessionCreateRequest>,
    ) -> Result<SessionResponse, SessionControllerError> {
        let user_auth = UserAuthModel {
            email: new_session.email,
            password: new_session.password,
        };

        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| SessionControllerError::InternalError)?;

        let session = UserAuthService::login(db_connection.as_mut(), &user_auth)
            .await
            .map_err(|err| match err {
                UserAuthServiceError::NotFoundError => SessionControllerError::InvalidCredentials,
                _ => SessionControllerError::InternalError,
            })?;

        let session_response = SessionResponse {
            id: session.id,
            token: session.token,
        };

        Ok(session_response)
    }

    pub async fn read(
        Extension(_state): Extension<Arc<AppState>>,
        Path((user_id, session_id)): Path<(Uuid, String)>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/users/{}/sessions/{}", user_id, session_id),
        )
    }

    pub async fn update(
        Extension(_state): Extension<Arc<AppState>>,
        Path((user_id, session_id)): Path<(Uuid, String)>,
        Json(_session_update_request): Json<SessionUpdateRequest>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/users/{}/sessions/{}", user_id, session_id),
        )
    }

    pub async fn delete(
        Extension(state): Extension<Arc<AppState>>,
        Path((_user_id, session_id)): Path<(Uuid, String)>,
    ) -> Result<SessionResponse, SessionControllerError> {
        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| SessionControllerError::InternalError)?;

        let session = UserAuthService::logout(db_connection.as_mut(), &session_id)
            .await
            .map_err(|err| match err {
                UserAuthServiceError::NotFoundError => SessionControllerError::SessionNotFound,
                _ => SessionControllerError::InternalError,
            })?;

        let session_response = SessionResponse {
            id: session.id,
            token: session.token,
        };

        Ok(session_response)
    }
}

pub enum SessionControllerError {
    InternalError,
    SessionNotFound,
    InvalidCredentials,
}

impl SessionControllerError {
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::InternalError => {
                "An error has occurred while processing your request. Please try again later."
            }
            Self::InvalidCredentials => "Invalid credentials",
            Self::SessionNotFound => "Session token not found",
        }
    }
}

impl IntoResponse for SessionControllerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_code()).into_response()
    }
}
