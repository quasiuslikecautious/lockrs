use serde::Serialize;
use uuid::Uuid;

use crate::models::{
    ValidatedClient,
    AuthToken,
};

/// The auth token provided when a client has successfully authorized through the grant flow
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token_type: String, // usually just 'Bearer'
    pub expires_in: i64,
    pub access_token: String, // 10 minutes
    pub refresh_token: String, // 24 hours
    pub scopes: String,
}

impl From<AuthToken> for TokenResponse {
    fn from(token: AuthToken) -> Self {
        TokenResponse {
            token_type: String::from("Bearer"),
            expires_in: 600,
            access_token: token.access,
            refresh_token: token.refresh,
            scopes: token.scopes,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ClientResponse {
    pub client_id: Uuid,
    pub client_secret: Option<String>,
}

impl From<ValidatedClient> for ClientResponse {
    fn from(client: ValidatedClient) -> Self {
        ClientResponse {
            client_id: client.get_id(),
            client_secret: client.get_secret(),
        }
    }
}


