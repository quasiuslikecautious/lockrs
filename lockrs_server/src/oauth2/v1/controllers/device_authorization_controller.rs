use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    oauth2::v1::{
        responses::DeviceAuthorizationResponse,
        services::{
            DeviceAuthorizationService, DeviceAuthorizationServiceError, ScopeService,
            ScopeServiceError,
        },
    },
    services::{ClientAuthService, ClientAuthServiceError},
    utils::extractors::ExtractClientCredentials,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct DeviceAuthorizationRequest {
    scope: String,
}

pub struct DeviceAuthorizationController;

impl DeviceAuthorizationController {
    pub async fn handle(
        State(state): State<AppState>,
        ExtractClientCredentials(client_credentials): ExtractClientCredentials,
        Query(params): Query<DeviceAuthorizationRequest>,
    ) -> Result<DeviceAuthorizationResponse, DeviceAuthorizationControllerError> {
        tracing::trace!(
            method = "handle",
            params = ?params
        );

        let db_context = &state.db_context;
        let client_auth_repository = &*state.repository_container.as_ref().client_auth_repository;

        ClientAuthService::authenticate(
            db_context,
            client_auth_repository,
            &client_credentials.id,
            client_credentials.secret.as_deref(),
        )
        .await
        .map_err(DeviceAuthorizationControllerError::from)?;

        let scope_repository = &*state.repository_container.as_ref().scope_repository;
        let scopes = ScopeService::get_from_list(db_context, scope_repository, &params.scope)
            .await
            .map_err(DeviceAuthorizationControllerError::from)?;

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
        .map_err(DeviceAuthorizationControllerError::from)?;

        Ok(DeviceAuthorizationResponse::new(
            &device_authorization.user_code,
            &device_authorization.device_code,
        ))
    }
}

pub enum DeviceAuthorizationControllerError {
    InvalidClient,
    InvalidScopes,

    BadRequest,
    InternalError,
}

impl DeviceAuthorizationControllerError {
    pub fn error_code(&self) -> StatusCode {
        match self {
            Self::InvalidClient => StatusCode::UNAUTHORIZED,

            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,

            _ => StatusCode::BAD_REQUEST,
        }
    }

    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InvalidClient => "The provided client credentials are invalid.",
            Self::InvalidScopes => "The provided scopes are invalid.",

            Self::BadRequest => "Unable to perform the requested operation.",
            Self::InternalError => {
                "An error occurred processing your request. Please try again later."
            }
        }
    }
}

impl From<DeviceAuthorizationServiceError> for DeviceAuthorizationControllerError {
    fn from(err: DeviceAuthorizationServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            DeviceAuthorizationServiceError::NotCreated => Self::BadRequest,

            _ => Self::InternalError,
        }
    }
}

impl From<ClientAuthServiceError> for DeviceAuthorizationControllerError {
    fn from(err: ClientAuthServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            ClientAuthServiceError::NotFound => Self::InvalidClient,
            _ => Self::InternalError,
        }
    }
}

impl From<ScopeServiceError> for DeviceAuthorizationControllerError {
    fn from(err: ScopeServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            ScopeServiceError::InvalidScopes => Self::InvalidScopes,
            _ => Self::InternalError,
        }
    }
}

impl IntoResponse for DeviceAuthorizationControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
