use std::sync::Arc;

use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;

use crate::{
    oauth2::{
        responses::DeviceAuthorizationResponse,
        services::{
            ClientAuthService, ClientAuthServiceError, DeviceAuthorizationService, ScopeService,
            ScopeServiceError,
        },
    },
    pg::get_connection_from_pool,
    utils::extractors::ExtractClientCredentials,
    AppState,
};

#[derive(Deserialize)]
pub struct DeviceAuthorizationRequest {
    scope: String,
}

pub struct DeviceAuthorizationController;

impl DeviceAuthorizationController {
    pub async fn handle(
        Extension(state): Extension<Arc<AppState>>,
        ExtractClientCredentials(client_credentials): ExtractClientCredentials,
        Query(params): Query<DeviceAuthorizationRequest>,
    ) -> Result<DeviceAuthorizationResponse, DeviceAuthorizationControllerError> {
        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| DeviceAuthorizationControllerError::InternalError)?;

        ClientAuthService::verify_credentials(
            db_connection.as_mut(),
            &client_credentials.id,
            &client_credentials.secret,
        )
        .await
        .map_err(|err| match err {
            ClientAuthServiceError::NotFoundError => {
                DeviceAuthorizationControllerError::InvalidClient
            }
            _ => DeviceAuthorizationControllerError::InternalError,
        })?;

        let scopes = ScopeService::get_from_list(db_connection.as_mut(), &params.scope)
            .await
            .map_err(|err| match err {
                ScopeServiceError::InvalidScopes => {
                    DeviceAuthorizationControllerError::InvalidScopes
                }
                _ => DeviceAuthorizationControllerError::InternalError,
            })?;

        let device_authorization = DeviceAuthorizationService::create_device_authorization(
            db_connection.as_mut(),
            &client_credentials.id,
            scopes,
        )
        .await
        .map_err(|_| DeviceAuthorizationControllerError::InternalError)?;

        Ok(DeviceAuthorizationResponse::new(
            &device_authorization.user_code,
            &device_authorization.device_code,
        ))
    }
}

pub enum DeviceAuthorizationControllerError {
    InternalError,
    InvalidClient,
    InvalidScopes,
}

impl DeviceAuthorizationControllerError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InternalError => {
                "An error occurred processing your request. Please try again later."
            }
            Self::InvalidClient => "The provided client credentials are invalid.",
            Self::InvalidScopes => "The provided scopes are invalid.",
        }
    }
}

impl IntoResponse for DeviceAuthorizationControllerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
