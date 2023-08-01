use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    api::v1::{
        models::{UserLoginCredentials, UserRegistration},
        responses::{SessionTokenResponse, UserResponse},
        services::{UserAuthService, UserAuthServiceError},
    },
    utils::extractors::BasicAuth,
    AppState,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

pub struct UserAuthController;

impl UserAuthController {
    pub async fn register(
        State(state): State<Arc<AppState>>,
        Json(register_request): Json<RegisterRequest>,
    ) -> Result<UserResponse, UserAuthControllerError> {
        tracing::trace!(method = "register_user", email = register_request.email,);

        let registration = UserRegistration {
            email: register_request.email,
            password: register_request.password,
        };

        let db_context = &state.as_ref().db_context;
        let user_auth_repository = &*state.repository_container.as_ref().user_auth_repository;

        let user = UserAuthService::register_user(db_context, user_auth_repository, &registration)
            .await
            .map_err(UserAuthControllerError::from)?;

        let user_response = UserResponse {
            id: user.id,
            email: user.email,
        };

        Ok(user_response)
    }

    pub async fn authenticate(
        State(state): State<Arc<AppState>>,
        BasicAuth(credentials): BasicAuth,
    ) -> Result<SessionTokenResponse, UserAuthControllerError> {
        tracing::trace!(method = "verify_credentials", email = credentials.public,);

        let auth = UserLoginCredentials {
            email: credentials.public,
            password: credentials.private,
        };

        let db_context = &state.as_ref().db_context;
        let user_auth_repository = &*state.repository_container.as_ref().user_auth_repository;
        let session_token_repository =
            &*state.repository_container.as_ref().session_token_repository;

        let session_token = UserAuthService::login(
            db_context,
            user_auth_repository,
            session_token_repository,
            &auth,
        )
        .await
        .map_err(UserAuthControllerError::from)?;

        let token_response = SessionTokenResponse {
            session_token: session_token.token,
            expires_at: session_token.expires_at,
        };

        Ok(token_response)
    }
}

pub enum UserAuthControllerError {
    InvalidCredentials,
    Internal,
}

impl UserAuthControllerError {
    pub fn error_code(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => "The provided credentials were invalid or not found.",
            Self::Internal => {
                "An error has occurred while proccessing your request. Please try again later."
            }
        }
    }
}

impl From<UserAuthServiceError> for UserAuthControllerError {
    fn from(err: UserAuthServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            UserAuthServiceError::Credentials => Self::InvalidCredentials,
            _ => Self::Internal,
        }
    }
}

impl IntoResponse for UserAuthControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
