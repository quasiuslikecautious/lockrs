use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    auth::{
        models::AuthModel,
        responses::SessionTokenResponse,
        services::{AuthService, AuthServiceError},
    },
    db, redis,
    shared::utils::extractors::BasicAuth,
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

        let mut db_connection = db::get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| AuthControllerError::Internal)?;

        let mut redis_connection = redis::get_connection_from_pool(&state.redis_pool)
            .await
            .map_err(|_| AuthControllerError::Internal)?;

        let session_token =
            AuthService::login(db_connection.as_mut(), redis_connection.as_mut(), &auth)
                .await
                .map_err(|err| match err {
                    AuthServiceError::Credentials => AuthControllerError::InvalidCredentials,
                    _ => AuthControllerError::Internal,
                })?;

        let token_response = SessionTokenResponse {
            session_token: session_token.token,
            expires_at: session_token.expires_at,
        };

        Ok(token_response)
    }
}

pub enum AuthControllerError {
    Internal,
    InvalidCredentials,
}

impl AuthControllerError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::Internal => {
                "An error has occurred while proccessing your request. Please try again later."
            }
            Self::InvalidCredentials => "The provided credentials were invalid or not found.",
        }
    }
}

impl IntoResponse for AuthControllerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
