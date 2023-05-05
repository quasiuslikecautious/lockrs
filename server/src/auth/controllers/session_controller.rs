use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth::services::{SessionService, SessionServiceError},
    db::get_connection_from_pool,
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
        Extension(_state): Extension<Arc<AppState>>,
        Json(_new_session): Json<SessionCreateRequest>,
    ) -> impl IntoResponse {
        (StatusCode::NOT_IMPLEMENTED, "/sessions".to_string())
    }

    pub async fn read(
        Extension(_state): Extension<Arc<AppState>>,
        Path(session_id): Path<String>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/sessions/{}", session_id),
        )
    }

    pub async fn update(
        Extension(_state): Extension<Arc<AppState>>,
        Path(session_id): Path<String>,
        Json(_session_update_request): Json<SessionUpdateRequest>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            format!("/sessions/{}", session_id),
        )
    }

    pub async fn delete(
        Extension(state): Extension<Arc<AppState>>,
        Path(session_id): Path<String>,
    ) -> Result<StatusCode, SessionControllerError> {
        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| SessionControllerError::InternalError)?;

        SessionService::delete_session(db_connection.as_mut(), &session_id)
            .await
            .map_err(|err| match err {
                SessionServiceError::NotFound => SessionControllerError::SessionNotFound,
                _ => SessionControllerError::InternalError,
            })?;

        Ok(StatusCode::NO_CONTENT)
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
