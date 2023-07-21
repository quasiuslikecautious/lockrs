use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    auth::{
        models::AuthModel,
        responses::SessionTokenResponse,
        services::{AuthService, AuthServiceError},
    },
    utils::extractors::BasicAuth,
    AppState,
};

pub struct AuthController;

impl AuthController {
    pub async fn auth(
        State(state): State<Arc<AppState>>,
        BasicAuth(credentials): BasicAuth,
    ) -> Result<SessionTokenResponse, AuthControllerError> {
        let auth = AuthModel {
            email: credentials.public,
            password: credentials.private,
        };

        let db_context = &state.as_ref().db_context;
        let user_repository = &*state.repository_container.as_ref().user_repository;
        let session_token_repository =
            &*state.repository_container.as_ref().session_token_repository;

        let session_token =
            AuthService::login(db_context, user_repository, session_token_repository, &auth)
                .await
                .map_err(AuthControllerError::from)?;

        let token_response = SessionTokenResponse {
            session_token: session_token.token,
            expires_at: session_token.expires_at,
        };

        Ok(token_response)
    }
}

pub enum AuthControllerError {
    InvalidCredentials,
    Internal,
}

impl AuthControllerError {
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

impl From<AuthServiceError> for AuthControllerError {
    fn from(err: AuthServiceError) -> Self {
        match err {
            AuthServiceError::Credentials(_) => Self::InvalidCredentials,
            _ => Self::Internal,
        }
    }
}

impl IntoResponse for AuthControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
