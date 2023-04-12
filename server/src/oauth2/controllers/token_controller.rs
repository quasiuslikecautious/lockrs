use axum::{
    extract::Query, 
    http::StatusCode,
    response::IntoResponse, Json, 
};
use serde::Deserialize;
use url::Url;


use crate::{
    models::ClientModel,
    oauth2::models::ScopesModel,
    oauth2::responses::TokenResponse, 
    oauth2::services::{
        ClientAuthService, ClientAuthServiceError, 
        ScopeService, ScopeServiceError, 
        TokenService, TokenServiceError,
    }, 
    utils::extractors::ExtractClientCredentials, 
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
        ExtractClientCredentials(client_credentials): ExtractClientCredentials,
        Query(params): Query<TokenRequest>,
    ) -> Result<Json<TokenResponse>, TokenControllerError> {
        let client = ClientAuthService::verify_credentials(
            &client_credentials.id, 
            &client_credentials.secret
        ).map_err(|err| {
            match err {
                ClientAuthServiceError::NotFoundError => TokenControllerError::InvalidClient,
                _ => TokenControllerError::InternalError,
            }
        })?;

        let scopes = ScopeService::get_from_list(params.scope.as_str())
            .map_err(|err| {
                match err {
                    ScopeServiceError::InvalidScopes => TokenControllerError::InvalidScopes,
                    _ => TokenControllerError::InternalError,
                }
            })?;

        let token: TokenResponse = match params.grant_type.as_str() {
            "authorization_code" => Self::authorization_code_token(),
            "urn:ietf:params:oauth:grant-type:device_code" => Self::device_authorization_token(),
            "client_credentials" => Self::client_credentials_token(client, scopes),
            "refresh_token" => Self::refresh_token(client, scopes, params),
            _ => Err(TokenControllerError::InvalidGrantType),
        }?;

        Ok(Json(token))
    }

    pub fn authorization_code_token() -> Result<TokenResponse, TokenControllerError> {
        // validate params
        // store/cache param info
        // redirect to frontend login with param info cache in params
        todo!();
    }

    pub fn device_authorization_token() -> Result<TokenResponse, TokenControllerError> {
        // validate params
        // handle polling
        //     exponential backoff
        // return token on acceptance criteria met
        todo!();
    }

    pub fn client_credentials_token(
        client: ClientModel,
        scopes: ScopesModel,
    ) -> Result<TokenResponse, TokenControllerError> {
        if client.secret == None {
            return Err(TokenControllerError::InvalidClient);
        }

         let token = TokenService::create_token(&client.id, &None, scopes)
             .map_err(|_| TokenControllerError::InternalError)?;

         Ok(token)
    }

    pub fn refresh_token(
        client: ClientModel, 
        scopes: ScopesModel,
        params: TokenRequest,
    ) -> Result<TokenResponse, TokenControllerError> {
        let Some(token) = params.refresh_token
        else {
            return Err(TokenControllerError::MissingRefreshToken);
        };

        let refresh_token = TokenService::verify_refresh_token(&client.id, token.as_str())
            .map_err(|err| {
                match err {
                    TokenServiceError::NotFound => TokenControllerError::InvalidRefreshToken,
                    _ => TokenControllerError::InternalError,
                }
            })?;

        TokenService::use_refresh_token(&client.id, &refresh_token.token)
            .map_err(|_| TokenControllerError::InternalError)?;

        TokenService::create_token(&client.id, &refresh_token.user_id, scopes)
            .map_err(|_| TokenControllerError::InternalError)
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

