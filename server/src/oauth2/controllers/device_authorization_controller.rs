use axum::{response::IntoResponse, extract::Query, http::StatusCode, Json};
use serde::Deserialize;

use crate::{
    oauth2::responses::DeviceAuthorizationResponse,
    oauth2::services::{
        ClientAuthService, ClientAuthServiceError, 
        DeviceAuthorizationService,
        ScopeService, ScopeServiceError,
    }, 
    utils::extractors::ExtractClientCredentials, 
};

#[derive(Deserialize)]
pub struct DeviceAuthorizationRequest {
    pub scope: String,
}

pub struct DeviceAuthorizationController;

impl DeviceAuthorizationController {
    pub async fn handle(
        ExtractClientCredentials(client_credentials): ExtractClientCredentials,
        Query(params): Query<DeviceAuthorizationRequest>
    ) -> Result<Json<DeviceAuthorizationResponse>, DeviceAuthorizationControllerError> {
        ClientAuthService::verify_credentials(
            &client_credentials.id,
            &client_credentials.secret,
        ).map_err(|err| {
            match err {
                ClientAuthServiceError::NotFoundError => DeviceAuthorizationControllerError::InvalidClient,
                _ => DeviceAuthorizationControllerError::InternalError,
            }
        })?;

        let scopes = ScopeService::get_from_list(&params.scope)
            .map_err(|err| {
                match err {
                    ScopeServiceError::InvalidScopes => DeviceAuthorizationControllerError::InvalidScopes,
                    _ => DeviceAuthorizationControllerError::InternalError,
                }
            })?;

        let device_authorization = DeviceAuthorizationService::create_device_authorization(&client_credentials.id, scopes)
            .map_err(|_| DeviceAuthorizationControllerError::InternalError)?;

        Ok(Json(DeviceAuthorizationResponse::new(&device_authorization.user_code, &device_authorization.device_code)))
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
            Self::InternalError => "An error occurred processing your request. Please try again later.",
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


