use std::collections::HashMap;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode}, 
    response::IntoResponse,
};
use crate::models::ClientAuthModel;

use super::BasicAuth;

#[derive(Debug)]
pub struct ExtractClientCredentials(pub ClientAuthModel);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractClientCredentials
where
    S: Send + Sync,
{
    type Rejection = ClientCredentialsError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> { 

        if let Some(client_credentials) = get_client_from_query(parts.uri.query()) {
            return Ok(Self(client_credentials));
        } 

        if let Some(BasicAuth(client_credentials)) = BasicAuth::from_request_parts(parts, state)
            .await
            .ok()
        { 
            return Ok(Self(ClientAuthModel {
                id: client_credentials.public,
                secret: Some(client_credentials.private),
            }));
        }

        Err(Self::Rejection::NotFound)
    }
}

pub enum ClientCredentialsError {
    NotFound,
}

impl ClientCredentialsError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::NotFound => "Client credentials missing from request.",
            
        }
    }
}

impl IntoResponse for ClientCredentialsError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}

fn query_into_hashmap(query: &str) -> HashMap<String, Option<String>> {
    query
        .split('&')
        .map(|phrase| {
            match phrase.split_once('=') {
                Some(pair) => (pair.0.to_owned(), Some(pair.1.to_owned())),
                None => (phrase.to_owned(), None),
            }
        })
        .collect::<HashMap<String, Option<String>>>()
}

fn get_client_from_query(query: Option<&str>) -> Option<ClientAuthModel> {
    let Some(query) = query
    else {
        return None;
    };

    let query_hash = query_into_hashmap(query);

    let Some(client_id) = query_hash.get("client_id")
    else {
        return None;
    };

    let Some(client_id_val) = client_id
    else {
        return None;
    };

    Some(ClientAuthModel {
        id: client_id_val.to_owned(),
        secret: None,
    })
}

