use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    api::v1::{
        models::AuthModel,
        responses::SessionTokenResponse,
        services::{UserAuthService, UserAuthServiceError},
    },
    utils::extractors::BasicAuth,
    AppState,
};

pub struct UserAuthController;

impl UserAuthController {
    pub async fn auth(
        State(state): State<Arc<AppState>>,
        BasicAuth(credentials): BasicAuth,
    ) -> Result<SessionTokenResponse, UserAuthControllerError> {
        tracing::trace!(method = "auth", email = credentials.public,);

        let auth = AuthModel {
            email: credentials.public,
            password: credentials.private,
        };

        let db_context = &state.as_ref().db_context;
        let user_repository = &*state.repository_container.as_ref().user_repository;
        let session_token_repository =
            &*state.repository_container.as_ref().session_token_repository;

        let session_token =
            UserAuthService::login(db_context, user_repository, session_token_repository, &auth)
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
