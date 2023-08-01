use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use url::Url;

use crate::{
    oauth2::v1::services::{
        ClientAuthService, ClientAuthServiceError, ScopeService, ScopeServiceError,
    },
    services::{RedirectService, RedirectServiceError},
    utils::extractors::ExtractClientCredentials,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct AuthorizeRequest {
    pub response_type: String,
    pub redirect_uri: Url,
    pub code_challenge: String,
    pub code_challenge_method: String,
    pub scope: String,
}

pub struct AuthorizeController;

impl AuthorizeController {
    pub async fn handle(
        State(state): State<Arc<AppState>>,
        ExtractClientCredentials(client_credentials): ExtractClientCredentials,
        Query(params): Query<AuthorizeRequest>,
    ) -> impl IntoResponse {
        tracing::trace!(
            method = "handle",
            params = ?params
        );

        if &params.response_type != "code" {
            tracing::error!(error = "Invalid Response Type Requested!");
            return Err(AuthorizeControllerError::InvalidResponseType);
        }

        let db_context = &state.as_ref().db_context;
        let client_repository = &*state.repository_container.as_ref().client_repository;

        let client = ClientAuthService::verify_credentials(
            db_context,
            client_repository,
            &client_credentials.id,
            client_credentials.secret.as_deref(),
        )
        .await
        .map_err(AuthorizeControllerError::from)?;

        // validate redirect uri, inform the user of the problem instead of redirecting
        let redirect_repository = &*state.repository_container.as_ref().redirect_repository;
        RedirectService::verify_redirect(
            db_context,
            redirect_repository,
            &client.id,
            &params.redirect_uri,
        )
        .await
        .map_err(AuthorizeControllerError::from)?;

        let scope_repository = &*state.repository_container.as_ref().scope_repository;
        let _scopes = ScopeService::get_from_list(db_context, scope_repository, &params.scope)
            .await
            .map_err(AuthorizeControllerError::from)?;

        let _is_plain = !params.code_challenge_method.eq("S256");

        // stash data before redirect

        Ok(Redirect::to(params.redirect_uri.as_str()))
    }
}

pub enum AuthorizeControllerError {
    InvalidResponseType,
    InvalidClient,
    InvalidRedirectUri,
    InvalidScopes,
    InvalidCodeChallengeMethod,

    InternalError,
}

impl AuthorizeControllerError {
    pub fn error_code(&self) -> StatusCode {
        match self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,

            _ => StatusCode::BAD_REQUEST,
        }
    }

    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InvalidResponseType => "The requested response type is invalid. Only the \"code\" response type is supported on this server.",
            Self::InvalidClient => "The provided client credentials are invalid.",
            Self::InvalidRedirectUri => "The provided redirect uri is not recognized by the server for the provided client.",
            Self::InvalidScopes => "The provided scopes are invalid.",
            Self::InvalidCodeChallengeMethod => "The provided code challenge method is unsupported. Only \"plain\" or \"S256\" code challenge methods are supported by this server",

            Self::InternalError => "An error occurred processing your request. Please try again later.",
        }
    }
}

impl From<ClientAuthServiceError> for AuthorizeControllerError {
    fn from(err: ClientAuthServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            ClientAuthServiceError::NotFound => Self::InvalidRedirectUri,
            _ => Self::InternalError,
        }
    }
}

impl From<RedirectServiceError> for AuthorizeControllerError {
    fn from(err: RedirectServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            RedirectServiceError::NotFound => Self::InvalidClient,
            _ => Self::InternalError,
        }
    }
}

impl From<ScopeServiceError> for AuthorizeControllerError {
    fn from(err: ScopeServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            ScopeServiceError::InvalidScopes => Self::InvalidScopes,
            _ => Self::InternalError,
        }
    }
}

impl IntoResponse for AuthorizeControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
