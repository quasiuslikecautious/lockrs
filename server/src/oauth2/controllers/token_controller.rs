use std::sync::Arc;

use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;
use url::Url;

use crate::{
    models::ClientModel,
    oauth2::models::ScopeModel,
    oauth2::responses::TokenResponse,
    oauth2::services::{
        ClientAuthService, RefreshTokenService, RefreshTokenServiceError, ScopeService,
        TokenService,
    },
    utils::extractors::ExtractClientCredentials,
    AppState,
};

#[derive(Deserialize)]
pub struct TokenRequest {
    // required
    pub grant_type: String,
    pub scope: String,

    // authorization code
    pub redirect_uri: Option<Url>,
    pub code: Option<String>,
    pub code_verifier: Option<String>,

    // device authorization
    pub device_code: Option<String>,

    // refresh token
    pub refresh_token: Option<String>,
}

pub struct TokenController;

impl TokenController {
    pub async fn handle(
        Extension(state): Extension<Arc<AppState>>,
        ExtractClientCredentials(client_credentials): ExtractClientCredentials,
        Query(params): Query<TokenRequest>,
    ) -> Result<TokenResponse, TokenControllerError> {
        let client_repository = &state.repository_container.as_ref().client_repository;

        let client = ClientAuthService::verify_credentials(
            client_repository,
            &client_credentials.id,
            &client_credentials.secret,
        )
        .await
        .map_err(|_| TokenControllerError::InvalidClient)?;

        let scope_repository = &state.repository_container.as_ref().scope_repository;

        let scopes = ScopeService::get_from_list(scope_repository, params.scope.as_str())
            .await
            .map_err(|_| TokenControllerError::InvalidScopes)?;

        let token: TokenResponse = match params.grant_type.as_str() {
            "authorization_code" => Self::authorization_code_token(state).await,
            "urn:ietf:params:oauth:grant-type:device_code" => {
                Self::device_authorization_token(state).await
            }
            "client_credentials" => Self::client_credentials_token(state, client, scopes).await,
            "refresh_token" => Self::refresh_token(state, client, scopes, params).await,
            _ => Err(TokenControllerError::InvalidGrantType),
        }?;

        Ok(token)
    }

    pub async fn authorization_code_token(
        _state: Arc<AppState>,
    ) -> Result<TokenResponse, TokenControllerError> {
        // validate params
        // store/cache param info
        // redirect to frontend login with param info cache in params
        todo!();
    }

    pub async fn device_authorization_token(
        _state: Arc<AppState>,
    ) -> Result<TokenResponse, TokenControllerError> {
        // validate params
        // handle polling
        //     exponential backoff
        // return token on acceptance criteria met
        todo!();
    }

    pub async fn client_credentials_token(
        state: Arc<AppState>,
        client: ClientModel,
        scopes: ScopeModel,
    ) -> Result<TokenResponse, TokenControllerError> {
        if client.secret.is_none() {
            return Err(TokenControllerError::InvalidClient);
        }

        let access_token_repository = &state.repository_container.as_ref().access_token_repository;

        let refresh_token_repository =
            &state.repository_container.as_ref().refresh_token_repository;

        let token = TokenService::create_token(
            access_token_repository,
            refresh_token_repository,
            &client.id,
            &None,
            scopes,
        )
        .await
        .map_err(|_| TokenControllerError::InternalError)?;

        Ok(TokenResponse {
            token_type: token.token_type,
            expires_in: token.expires_in,
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            scopes: token.scopes,
        })
    }

    pub async fn refresh_token(
        state: Arc<AppState>,
        client: ClientModel,
        scopes: ScopeModel,
        params: TokenRequest,
    ) -> Result<TokenResponse, TokenControllerError> {
        let Some(token) = params.refresh_token
        else {
            return Err(TokenControllerError::MissingRefreshToken);
        };

        let refresh_token_repository =
            &state.repository_container.as_ref().refresh_token_repository;

        let refresh_token =
            RefreshTokenService::use_token(refresh_token_repository, token.as_str())
                .await
                .map_err(|err| match err {
                    RefreshTokenServiceError::NotFound => TokenControllerError::InvalidRefreshToken,
                    _ => TokenControllerError::InternalError,
                })?;

        let access_token_repository = &state.repository_container.as_ref().access_token_repository;

        let token = TokenService::create_token(
            access_token_repository,
            refresh_token_repository,
            &client.id,
            &refresh_token.user_id,
            scopes,
        )
        .await
        .map_err(|_| TokenControllerError::InternalError)?;

        Ok(TokenResponse {
            token_type: token.token_type,
            expires_in: token.expires_in,
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            scopes: token.scopes,
        })
    }
}

pub enum TokenControllerError {
    InternalError,
    InvalidClient,
    InvalidGrantType,
    InvalidScopes,
    MissingRefreshToken,
    InvalidRefreshToken,
}

impl TokenControllerError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InternalError => "An error has occurred while processing your request. Please try again later.",
            Self::InvalidClient => "The provided client is invalid.",
            Self::InvalidGrantType => "The provided grant_type is invalid. This server supports \"authorization_code\", \"urn:ietf:params:oauth:grant-type:device_code\", \"client_credentials\", and \"refresh_token.\"",
            Self::InvalidScopes => "The provided scopes are invalid.",
            Self::MissingRefreshToken => "The request is missing the \"refresh_token\" parameter.",
            Self::InvalidRefreshToken => "The provided refresh_token is invalid.",
        }
    }
}

impl IntoResponse for TokenControllerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
