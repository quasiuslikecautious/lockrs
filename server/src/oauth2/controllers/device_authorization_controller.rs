use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    oauth2::{
        responses::DeviceAuthorizationResponse,
        services::{ClientAuthService, DeviceAuthorizationService, ScopeService},
    },
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
        State(state): State<Arc<AppState>>,
        ExtractClientCredentials(client_credentials): ExtractClientCredentials,
        Query(params): Query<DeviceAuthorizationRequest>,
    ) -> Result<DeviceAuthorizationResponse, DeviceAuthorizationControllerError> {
        let db_context = &state.as_ref().db_context;
        let client_repository = &*state.repository_container.as_ref().client_repository;

        ClientAuthService::verify_credentials(
            db_context,
            client_repository,
            &client_credentials.id,
            client_credentials.secret.as_deref(),
        )
        .await
        .map_err(|_| DeviceAuthorizationControllerError::InvalidClient)?;

        let scope_repository = &*state.repository_container.as_ref().scope_repository;
        let scopes = ScopeService::get_from_list(db_context, scope_repository, &params.scope)
            .await
            .map_err(|_| DeviceAuthorizationControllerError::InvalidScopes)?;

        let device_authorization_repository = &*state
            .repository_container
            .as_ref()
            .device_authorization_repository;

        let device_authorization = DeviceAuthorizationService::create_device_authorization(
            db_context,
            device_authorization_repository,
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
