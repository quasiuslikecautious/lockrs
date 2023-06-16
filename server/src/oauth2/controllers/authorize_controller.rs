use std::sync::Arc;

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};
use serde::Deserialize;
use url::Url;

use crate::{
    db::get_connection_from_pool,
    oauth2::services::{ClientAuthService, ScopeService, ScopeServiceError},
    services::{RedirectService, RedirectServiceError},
    utils::extractors::ExtractClientCredentials,
    AppState,
};

#[derive(Deserialize)]
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
        Extension(state): Extension<Arc<AppState>>,
        ExtractClientCredentials(client_credentials): ExtractClientCredentials,
        Query(params): Query<AuthorizeRequest>,
    ) -> impl IntoResponse {
        if &params.response_type != "code" {
            return Err(AuthorizeControllerError::InvalidResponseType);
        }

        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| AuthorizeControllerError::InternalError)?;

        let client = ClientAuthService::verify_credentials(
            db_connection.as_mut(),
            &client_credentials.id,
            &client_credentials.secret,
        )
        .await
        .map_err(|_| AuthorizeControllerError::InvalidClient)?;

        // validate redirect uri, inform the user of the problem instead of redirecting
        RedirectService::verify_redirect(db_connection.as_mut(), &client.id, &params.redirect_uri)
            .await
            .map_err(|err| match err {
                RedirectServiceError::DbError => AuthorizeControllerError::InternalError,
                RedirectServiceError::NotFound => AuthorizeControllerError::InvalidRedirectUri,
            })?;

        let _scopes = ScopeService::get_from_list(db_connection.as_mut(), &params.scope)
            .await
            .map_err(|err| match err {
                ScopeServiceError::DbError => AuthorizeControllerError::InternalError,
                ScopeServiceError::InvalidScopes => AuthorizeControllerError::InvalidScopes,
            })?;

        let _is_plain = !params.code_challenge_method.eq("S256");

        // stash data before redirect

        Ok(Redirect::to(params.redirect_uri.as_str()))
    }
}

pub enum AuthorizeControllerError {
    InternalError,
    InvalidResponseType,
    InvalidClient,
    InvalidRedirectUri,
    InvalidScopes,
    InvalidCodeChallengeMethod,
}

impl AuthorizeControllerError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InternalError => "An error occurred processing your request. Please try again later.",
            Self::InvalidResponseType => "The requested response type is invalid. Only the \"code\" response type is supported on this server.",
            Self::InvalidClient => "The provided client credentials are invalid.",
            Self::InvalidRedirectUri => "The provided redirect uri is not recognized by the server for the provided client.",
            Self::InvalidScopes => "The provided scopes are invalid.",
            Self::InvalidCodeChallengeMethod => "The provided code challenge method is unsupported. Only \"plain\" or \"S256\" code challenge methods are supported by this server",
        }
    }
}

impl IntoResponse for AuthorizeControllerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
