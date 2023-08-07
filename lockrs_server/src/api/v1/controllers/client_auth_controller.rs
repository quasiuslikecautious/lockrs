use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use url::Url;

use crate::{
    api::v1::responses::ClientResponse,
    models::ClientRegistration,
    services::{ClientAuthService, ClientAuthServiceError},
    utils::extractors::SessionJwt,
    AppState,
};

pub struct ClientAuthController;

#[derive(Debug, Deserialize)]
pub struct ClientCreateRequest {
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub homepage_url: Url,
    pub redirect_url: Url,
}

impl ClientAuthController {
    pub async fn register(
        State(state): State<AppState>,
        SessionJwt(auth_info): SessionJwt,
        Json(new_client_request): Json<ClientCreateRequest>,
    ) -> Result<ClientResponse, ClientAuthControllerError> {
        tracing::trace!(
            method = "register",
            params = ?new_client_request
        );

        let new_client = ClientRegistration {
            user_id: auth_info.user_id,
            is_public: new_client_request.is_public,
            name: new_client_request.name,
            description: new_client_request.description,
            homepage_url: new_client_request.homepage_url,
            redirect_url: new_client_request.redirect_url,
        };

        let db_context = &state.db_context;
        let client_auth_repository = &*state.repository_container.as_ref().client_auth_repository;

        let client = ClientAuthService::register(db_context, client_auth_repository, new_client)
            .await
            .map_err(ClientAuthControllerError::from)?;

        Ok(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        })
    }
}

pub enum ClientAuthControllerError {
    BadRequest,
    Internal,
}

impl ClientAuthControllerError {
    pub fn error_code(&self) -> StatusCode {
        match self {
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    pub fn error_message(&self) -> &'static str {
        match self {
            Self::BadRequest => "Unable to perform the requested operation.",
            Self::Internal => {
                "An error has occurred while processing your request. Please try again later."
            }
        }
    }
}

impl From<ClientAuthServiceError> for ClientAuthControllerError {
    fn from(err: ClientAuthServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            ClientAuthServiceError::InternalError => Self::Internal,
            _ => Self::BadRequest,
        }
    }
}

impl IntoResponse for ClientAuthControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
