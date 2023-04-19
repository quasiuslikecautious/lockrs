use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{
    auth::{
        models::AuthModel,
        responses::AuthResponse,
        services::{AuthService, AuthServiceError},
    },
    db::get_connection_from_pool,
    AppState,
};

pub struct AuthController;

pub struct AuthCreateRequest {
    pub email: String,
    pub password: String,
}

impl AuthController {
    pub async fn create(
        Extension(state): Extension<Arc<AppState>>,
        Json(new_session): Json<AuthCreateRequest>,
    ) -> Result<AuthResponse, AuthControllerError> {
        let auth = AuthModel {
            email: new_session.email,
            password: new_session.password,
        };

        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| AuthControllerError::Internal)?;

        let session = AuthService::login(db_connection.as_mut(), state.jwt_util.as_ref(), &auth)
            .await
            .map_err(|err| match err {
                AuthServiceError::NotFound => AuthControllerError::InvalidCredentials,
                _ => AuthControllerError::Internal,
            })?;

        let session_response = AuthResponse {
            session_token: session.token,
        };

        Ok(session_response)
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
