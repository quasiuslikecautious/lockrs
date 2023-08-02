use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};
use ring::rand::{SecureRandom, SystemRandom};
use thiserror::Error;

use crate::{
    db::{
        repositories::{ClientAuthRepository, QueryFailure, RepositoryError},
        DbContext,
    },
    mappers::ClientAuthMapper,
    models::{ClientAuthModel, ClientModel, ClientRegistration, RedirectCreateModel},
};

pub struct ClientAuthService;

impl ClientAuthService {
    pub async fn register(
        db_context: &Arc<DbContext>,
        client_auth_repository: &dyn ClientAuthRepository,
        new_client: ClientRegistration,
    ) -> Result<ClientModel, ClientAuthServiceError> {
        tracing::trace!(
            method = "register",
            user_id = new_client.user_id.to_string(),
            name = new_client.name,
        );

        let id = Self::generate_random_string();
        let secret = match new_client.is_public {
            true => None,
            false => Some(Self::generate_random_string()),
        };

        let client_create = ClientAuthModel {
            user_id: new_client.user_id,
            id: id.clone(),
            secret,
            name: new_client.name,
            description: new_client.description,
            homepage_url: new_client.homepage_url.to_string(),
        };

        let redirect_create = RedirectCreateModel {
            client_id: id,
            uri: new_client.redirect_url,
        };

        let client = client_auth_repository
            .create(db_context, &client_create, &redirect_create)
            .await
            .map_err(ClientAuthServiceError::from)?;

        tracing::info!(
            "Client created: {{ id: {}, name: {} }}",
            client.id,
            client.name,
        );

        Ok(ClientAuthMapper::into_client(client))
    }

    pub async fn authenticate(
        db_context: &Arc<DbContext>,
        client_auth_repository: &dyn ClientAuthRepository,
        id: &str,
        secret: Option<&str>,
    ) -> Result<ClientModel, ClientAuthServiceError> {
        tracing::trace!(method = "verify_credentials", id);

        let client = client_auth_repository
            .get_by_credentials(db_context, id, secret)
            .await
            .map_err(ClientAuthServiceError::from)?;

        tracing::info!("Client authenticated with ID: {}", id);

        Ok(ClientAuthMapper::into_client(client))
    }

    pub fn generate_random_string() -> String {
        let mut buffer = [0u8; 24];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer)
    }
}

#[derive(Debug, Error)]
pub enum ClientAuthServiceError {
    #[error("CLIENT AUTH SERVICE ERROR :: Not found")]
    NotFound,

    #[error("CLIENT AUTH SERVICE ERROR :: Internal Error")]
    InternalError,
}

impl From<RepositoryError> for ClientAuthServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            RepositoryError::QueryFailed(query_err) => match query_err {
                QueryFailure::NotFound => Self::NotFound,

                _ => Self::InternalError,
            },

            RepositoryError::InternalError => Self::InternalError,
        }
    }
}
