use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};
use ring::rand::{SecureRandom, SystemRandom};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    db::{
        repositories::{ClientRepository, RepositoryError}, 
        DbContext,
    },
    models::{ClientCreateModel, ClientModel, ClientUpdateModel, RedirectCreateModel},
};

pub struct ClientService;

impl ClientService {
    pub async fn create_client(
        db_context: &Arc<DbContext>,
        client_repository: &dyn ClientRepository,
        new_client: ClientCreateModel,
    ) -> Result<ClientModel, ClientServiceError> {
        let id = Self::generate_random_string();
        let secret = match new_client.is_public {
            true => None,
            false => Some(Self::generate_random_string()),
        };

        let client_create = ClientModel {
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

        client_repository
            .create(db_context, &client_create, &redirect_create)
            .await
            .map_err(ClientServiceError::from)
    }

    pub async fn get_client_by_id(
        db_context: &Arc<DbContext>,
        client_repository: &dyn ClientRepository,
        id: &str,
    ) -> Result<ClientModel, ClientServiceError> {
        client_repository
            .get_by_id(db_context, id)
            .await
            .map_err(ClientServiceError::from)
    }

    pub async fn get_clients_by_user(
        db_context: &Arc<DbContext>,
        client_repository: &dyn ClientRepository,
        user_id: &Uuid,
    ) -> Result<Vec<ClientModel>, ClientServiceError> {
        client_repository
            .get_all_by_user_id(db_context, user_id)
            .await
            .map_err(ClientServiceError::from)
    }

    pub async fn update_client_by_id(
        db_context: &Arc<DbContext>,
        client_repository: &dyn ClientRepository,
        client_id: &str,
        update_client: &ClientUpdateModel,
    ) -> Result<ClientModel, ClientServiceError> {
        client_repository
            .update_by_id(db_context, client_id, update_client)
            .await
            .map_err(ClientServiceError::from)
    }

    pub async fn delete_client_by_id(
        db_context: &Arc<DbContext>,
        client_repository: &dyn ClientRepository,
        client_id: &str,
    ) -> Result<(), ClientServiceError> {
        client_repository
            .delete_by_id(db_context, client_id)
            .await
            .map_err(ClientServiceError::from)
    }

    pub fn generate_random_string() -> String {
        let mut buffer = [0u8; 24];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer)
    }
}

#[derive(Debug, Error)]
pub enum ClientServiceError {
    #[error("CLIENT SERVICE ERROR :: Already Exists :: {0}")]
    AlreadyExists(String),
    #[error("CLIENT SERVICE ERROR :: Not Created :: {0}")]
    NotCreated(String),
    #[error("CLIENT SERVICE ERROR :: Not Found :: {0}")]
    NotFound(String),
    #[error("CLIENT SERVICE ERROR :: Not Updated :: {0}")]
    NotUpdated(String),
    #[error("CLIENT SERVICE ERROR :: Bad Deletion :: {0}")]
    BadDelete(String),

    #[error("CLIENT SERVICE ERROR :: Repository Error")]
    Repository(String),
}

impl From<RepositoryError> for ClientServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::AlreadyExists(msg) => Self::AlreadyExists(msg), 
            RepositoryError::NotCreated(msg) => Self::NotCreated(msg), 
            RepositoryError::NotFound(msg) => Self::NotFound(msg), 
            RepositoryError::NotUpdated(msg) => Self::NotUpdated(msg), 
            RepositoryError::NotDeleted(msg) => Self::BadDelete(msg), 

            RepositoryError::Connection(msg) => Self::Repository(msg),
            RepositoryError::Database(msg) => Self::Repository(msg),
        }
    }
}
