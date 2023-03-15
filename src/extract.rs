use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{
        header::AUTHORIZATION,
        request::Parts,
    },
};
use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    auth_response,
    models,
};

#[derive(Deserialize)]
struct ClientIdQueryParam {
    pub client_id: String,
}

#[derive(Debug)]
pub struct ExtractClientCredentials(pub models::ValidatedClient);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractClientCredentials
where
    S: Send + Sync,
{
    type Rejection = auth_response::Rejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = &parts.headers.get(AUTHORIZATION).map(|val| val.to_str().unwrap());

        let mut unvalidated_client: Option<models::UnvalidatedClient> = None;

        if auth_header.is_some() {
            unvalidated_client = Some(extract_credentials_from_header(auth_header.unwrap())?);
        } else {
            let Some(query) = &parts.uri.query()
            else {
                return Err(Self::Rejection::InvalidClientId) 
            };

            unvalidated_client = Some(extract_credentials_from_query(query)?);
        }

        let validated_client = unvalidated_client.unwrap().validate()?;
        Ok(ExtractClientCredentials(validated_client))
    }
}

fn extract_credentials_from_header(client_auth: &str) -> auth_response::Result<models::UnvalidatedClient> {
    let Some((token_type, token)) = client_auth.split_once(' ')
    else {
        return Err(auth_response::Rejection::InvalidClientId);
    };

    if token_type != "Bearer" {
        return Err(auth_response::Rejection::InvalidClientId);
    }

    let client_auth_bytes = general_purpose::URL_SAFE_NO_PAD
        .decode::<&str>(token)
        .map_err(|_| auth_response::Rejection::InvalidClientId)?;

    let client_auth_str = String::from_utf8(client_auth_bytes)
        .map_err(|_| auth_response::Rejection::InvalidClientId)?;

    let Some((client_id, client_secret)) = client_auth_str.split_once(':')
    else {
        return Err(auth_response::Rejection::InvalidClientId);
    };

    let client_uuid = Uuid::parse_str(client_id)
        .map_err(|_| auth_response::Rejection::InvalidClientId)?;

    let unvalidated_client = models::UnvalidatedClient::new(
        &client_uuid,
        Some(client_secret.to_string())
    );

    Ok(unvalidated_client)
}

fn extract_credentials_from_query(query: &str) -> auth_response::Result<models::UnvalidatedClient> {
    let Some(client_id_param) = serde_urlencoded::from_str::<ClientIdQueryParam>(query).ok()
    else {
        return Err(auth_response::Rejection::InvalidClientId) 
    };

    let Some(client_uuid) = Uuid::parse_str(&client_id_param.client_id).ok()
    else {
        return Err(auth_response::Rejection::InvalidClientId);
    };

    let unvalidated_client = models::UnvalidatedClient::new(
        &client_uuid,
        None,
    );

    Ok(unvalidated_client)
}

