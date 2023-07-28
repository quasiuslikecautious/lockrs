use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use url::Url;

use crate::{
    models::ClientModel,
    oauth2::v1::models::ScopeModel,
    oauth2::v1::services::{
        ClientAuthService, ClientAuthServiceError, RefreshTokenService, RefreshTokenServiceError,
        ScopeService, TokenService, ScopeServiceError,
    },
    oauth2::v1::{responses::TokenResponse, services::TokenServiceError},
    utils::extractors::ExtractClientCredentials,
    AppState,
};

#[derive(Debug, Deserialize)]
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
        State(state): State<Arc<AppState>>,
        ExtractClientCredentials(client_credentials): ExtractClientCredentials,
        Query(params): Query<TokenRequest>,
    ) -> Result<TokenResponse, TokenControllerError> {
        tracing::trace!(
            method = "handle",
            params = ?params
        );

        let db_context = &state.as_ref().db_context;
        let client_repository = &*state.repository_container.as_ref().client_repository;

        let client = ClientAuthService::verify_credentials(
            db_context,
            client_repository,
            client_credentials.id.as_str(),
            client_credentials.secret.as_deref(),
        )
        .await
        .map_err(TokenControllerError::from)?;

        let scope_repository = &*state.repository_container.as_ref().scope_repository;

        let scopes =
            ScopeService::get_from_list(db_context, scope_repository, params.scope.as_str())
                .await
                .map_err(TokenControllerError::from)?;

        let token: TokenResponse = match params.grant_type.as_str() {
            "authorization_code" => Self::authorization_code_token(state).await,
            "urn:ietf:params:oauth:grant-type:device_code" => {
                Self::device_authorization_token(state).await
            }
            "client_credentials" => Self::client_credentials_token(state, client, scopes).await,
            "refresh_token" => Self::refresh_token(state, client, scopes, params).await,
            _ => {
                tracing::error!(error = "Invalid grant type supplied.");
                Err(TokenControllerError::InvalidGrantType)
            },
        }?;

        Ok(token)
    }

    pub async fn authorization_code_token(
        _state: Arc<AppState>,
    ) -> Result<TokenResponse, TokenControllerError> {
        tracing::trace!(
            method = "authorization_code_token"
        );

        // validate params
        // store/cache param info
        // redirect to frontend login with param info cache in params
        todo!();
    }

    pub async fn device_authorization_token(
        _state: Arc<AppState>,
    ) -> Result<TokenResponse, TokenControllerError> {
        tracing::trace!(
            method = "device_authorization_token"
        );

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
        tracing::trace!(
            method = "client_credentials_token",
            client = client.id,
            scopes = ?scopes
        );

        if client.secret.is_none() {
            tracing::error!(error = "Missing client secret");
            return Err(TokenControllerError::InvalidClient);
        }

        let db_context = &state.as_ref().db_context;
        let access_token_repository = &*state.repository_container.as_ref().access_token_repository;
        let refresh_token_repository =
            &*state.repository_container.as_ref().refresh_token_repository;

        let token = TokenService::create_token(
            db_context,
            access_token_repository,
            refresh_token_repository,
            &client.id,
            None,
            scopes,
        )
        .await
        .map_err(TokenControllerError::from)?;

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
        tracing::trace!(
            method = "refresh_token",
            client = client.id,
            scopes = ?scopes,
            params = ?params
        );

        let Some(token) = params.refresh_token
        else {
            tracing::error!(error = "Missing refresh token in request");
            return Err(TokenControllerError::MissingRefreshToken);
        };

        let db_context = &state.as_ref().db_context;
        let refresh_token_repository =
            &*state.repository_container.as_ref().refresh_token_repository;

        let refresh_token =
            RefreshTokenService::use_token(db_context, refresh_token_repository, token.as_str())
                .await
                .map_err(TokenControllerError::from)?;

        let access_token_repository = &*state.repository_container.as_ref().access_token_repository;

        let token = TokenService::create_token(
            db_context,
            access_token_repository,
            refresh_token_repository,
            &client.id,
            refresh_token.user_id.as_ref(),
            scopes,
        )
        .await
        .map_err(TokenControllerError::from)?;

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
    InvalidClient,
    InvalidGrantType,
    InvalidScopes,
    MissingRefreshToken,
    InvalidRefreshToken,

    BadRequest,
    InternalError,
}

impl TokenControllerError {
    pub fn error_code(&self) -> StatusCode {
        match self {
            Self::InvalidClient => StatusCode::UNAUTHORIZED,

            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,

            _ => StatusCode::BAD_REQUEST,
        }
    }

    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InvalidClient => "The provided client is invalid.",
            Self::InvalidGrantType => "The provided grant_type is invalid. This server supports \"authorization_code\", \"urn:ietf:params:oauth:grant-type:device_code\", \"client_credentials\", and \"refresh_token.\"",
            Self::InvalidScopes => "The provided scopes are invalid.",
            Self::MissingRefreshToken => "The request is missing the \"refresh_token\" parameter.",
            Self::InvalidRefreshToken => "The provided refresh_token is invalid.",

            Self::BadRequest => "Unable to perform the requested operation.",
            Self::InternalError => "An error has occurred while processing your request. Please try again later.",
        }
    }
}

impl From<TokenServiceError> for TokenControllerError {
    fn from(err: TokenServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            TokenServiceError::NotCreated(_) => Self::BadRequest,
            TokenServiceError::InternalError(_) => Self::InternalError,
        }
    }
}

impl From<RefreshTokenServiceError> for TokenControllerError {
    fn from(err: RefreshTokenServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            RefreshTokenServiceError::NotFound(_) => Self::InvalidRefreshToken,
            _ => Self::InternalError,
        }
    }
}

impl From<ClientAuthServiceError> for TokenControllerError {
    fn from(err: ClientAuthServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            ClientAuthServiceError::NotFound(_) => Self::InvalidClient,
            _ => Self::InternalError,
        }
    }
}

impl From<ScopeServiceError> for TokenControllerError {
    fn from(err: ScopeServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            ScopeServiceError::InvalidScopes(_) => Self::InvalidClient,
            _ => Self::InternalError,
        }
    }
}

impl IntoResponse for TokenControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
