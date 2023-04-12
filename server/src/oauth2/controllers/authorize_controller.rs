use axum::{response::{IntoResponse, Redirect}, extract::Query, http::StatusCode};
use serde::Deserialize;
use url::Url;

use crate::{
    oauth2::services::{
        ClientAuthService,
        ScopeService, ScopeServiceError,
    },
    services::{
        RedirectService, RedirectServiceError, 
    },
    utils::extractors::ExtractClientCredentials, 
};

#[derive(Debug, Deserialize)]
pub struct AuthorizeRequest {
    pub response_type: String,
    pub redirect_uri: Url,
    pub scope: String,
    pub state: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
}
pub struct AuthorizeController;

impl AuthorizeController {
    pub async fn handle(
        ExtractClientCredentials(client_credentials): ExtractClientCredentials,
        Query(params): Query<AuthorizeRequest>,
    ) -> impl IntoResponse {
        if &params.response_type != "code" {
            return Err(AuthorizeControllerError::InvalidResponseType)
        }
        
        let client = ClientAuthService::verify_credentials(
            &client_credentials.id,
            &client_credentials.secret
        ).map_err(|_| AuthorizeControllerError::InvalidClient)?;
    
        // validate redirect uri, inform the user of the problem instead of redirecting
        RedirectService::verify_redirect(&client.id, &params.redirect_uri)
            .map_err(|err| {
                match err {
                    RedirectServiceError::DbError => AuthorizeControllerError::InternalError,
                    RedirectServiceError::NotFound => AuthorizeControllerError::InvalidRedirectUri,
                }
            })?;
    
        let scopes = ScopeService::get_from_list(&params.scope)
            .map_err(|err| {
                match err {
                    ScopeServiceError::DbError => AuthorizeControllerError::InternalError,
                    ScopeServiceError::InvalidScopes => AuthorizeControllerError::InvalidScopes,
                }
            })?;
        
    
        let is_plain = !params.code_challenge_method.eq("S256");

        // stash data before redirect

        Ok(Redirect::to("/login"))
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

