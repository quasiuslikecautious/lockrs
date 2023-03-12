use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{
        header::AUTHORIZATION,
        request::Parts
    },
};
use base64::{Engine as _, engine::general_purpose,};
use serde_urlencoded;

use crate::{ auth_response::{self, Rejection}, models::{self, ClientType} };

pub enum GrantType {
    AuthorizationCodeGrant,
    ClientCredentialsGrant,
    DeviceCodeGrant,
    RefreshTokenGrant,
}

impl GrantType {
    pub fn from_string(value: &str) -> Option<Self> {
        match value {
            "authorization_code" => {
                Some(Self::AuthorizationCodeGrant)
            },
            "client_credentials" => {
                Some(Self::ClientCredentialsGrant)
            },
            "device_code" => {
                Some(Self::DeviceCodeGrant)
            },
            "refresh_token" => {
                Some(Self::RefreshTokenGrant)
            },
            _ => None,
        }
    }

    pub fn into_token(&self, query: &str, client: models::Client) -> Result<models::Token, auth_response::Rejection> {
        let query = match self {
            GrantType::AuthorizationCodeGrant => {
                println!("auth code grant");
                let params = serde_urlencoded::from_str::<models::AuthorizationCodeParams>(query)
                    .map_err(|_| auth_response::Rejection::InvalidRequest)?;

                println!("{:?}", params);
                
                params.grant_type
            },
            GrantType::ClientCredentialsGrant => {
                println!("client cred grant");
                let params = serde_urlencoded::from_str::<models::ClientCredentialsParams>(query)
                    .map_err(|_| auth_response::Rejection::InvalidRequest)?;

                println!("{:?}", params);

                params.grant_type
            },
            GrantType::DeviceCodeGrant => {
                println!("device code grant");
                let params = serde_urlencoded::from_str::<models::DeviceCodeParams>(query)
                    .map_err(|_| auth_response::Rejection::InvalidRequest)?;
                
                println!("{:?}", params);

                params.grant_type
            },
            GrantType::RefreshTokenGrant => {
                println!("refresh token grant");
                let params = serde_urlencoded::from_str::<models::RefreshTokenParams>(query)
                    .map_err(|_| auth_response::Rejection::InvalidRequest)?;

                println!("{:?}", params);

                params.grant_type
            },
        };

        Ok(models::Token::new(
            String::from("Bearer"),
            2023,
            query,
            Some(String::from("token.refresh")),
            vec![String::from("scope.admin")]
        ))
    }
}

pub struct ExtractTokenFromGrant(pub Result<models::Token, auth_response::Rejection>);

impl ExtractTokenFromGrant {
    pub fn extract_client(
        parts: &Parts
    ) -> Option<models::Client> {
        let Some(client_auth) = &parts.headers
            .get(AUTHORIZATION)
            .map(|val| val.to_str().unwrap())
        else {
            // grab from query
            let Some(query) = &parts.uri.query()
            else {
                return None;
            };

            let Some(client_id_param) 
                = serde_urlencoded::from_str::<models::ClientIdQueryParam>(query).ok()
            else {
                return None;
            };

            return Some(
                models::Client::new(
                    &client_id_param.client_id,
                    None
                )
            );
        };

        let Some((token_type, token)) = client_auth.split_once(' ') 
        else {
            return None;
        };

        let Some(client_auth_bytes) = general_purpose::URL_SAFE_NO_PAD
            .decode::<&str>(token)
            .ok()
        else {
            return None;
        };

        let Some(client_auth_str) = String::from_utf8(client_auth_bytes).ok()
        else {
            return None;
        };

        if let Some((client_id, client_secret)) = client_auth_str.split_once(':') {
            return Some(
                models::Client::new(
                    client_id,
                    Some(client_secret.to_string())
                )
            );
        } else {
            return None;
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ExtractTokenFromGrant
where
    S: Send + Sync,
{
    type Rejection = auth_response::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Some(client) = Self::extract_client(&parts)
        else {
            return Err(Rejection::UnsupportedGrantType);
        };

        println!("{:?}", client);

        let query = match parts.uri.query() {
            Some(val) => val,
            None => return Err(Rejection::InvalidRequest),
        };

        let Some(grant_type_param) 
            = serde_urlencoded::from_str::<models::GrantTypeQueryParam>(query).ok()
        else {
            return Err(Rejection::InvalidRequest);
        };
    
        if let Some(grant) = GrantType::from_string(grant_type_param.grant_type.as_str()) {
            return Ok(
                Self(grant.into_token(query, client))
            );
        } else {
            return Err(Rejection::UnsupportedGrantType);
        }
    }
}
    

