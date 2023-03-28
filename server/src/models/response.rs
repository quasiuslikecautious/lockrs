use serde::Serialize;
use url::Url;

use crate::{
    models::Client,
    tokens::AuthToken,
};

#[derive(Debug, Serialize)]
pub struct AuthCodeResponse {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize)]
pub struct ClientResponse {
    pub client_id: String,
    pub client_secret: Option<String>,
}

impl From<Client> for ClientResponse {
    fn from(client: Client) -> Self {
        ClientResponse {
            client_id: client.get_id(),
            client_secret: client.get_secret(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DeviceCodeResponse {
    pub user_code: String,
    pub device_code: String,
    pub verification_uri: Url,
    pub interval: i32,
    pub expires_in: i32,
}

impl DeviceCodeResponse {
    pub fn new(
        user_code: String,
        device_code: String,
        verification_uri: Url,
    ) -> Self {
        Self {
            user_code,
            device_code,
            verification_uri,
            interval: 5,
            expires_in: 300,
        }
    }
}

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

