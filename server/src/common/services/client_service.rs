use base64::{engine::general_purpose, Engine as _};
use ring::rand::{SecureRandom, SystemRandom};
use uuid::Uuid;

use crate::{
    models::{ClientCreateModel, ClientModel, ClientUpdateModel, RedirectCreateModel},
    repositories::{ClientRepository, RedirectUriRepository},
};

pub struct ClientService;

impl ClientService {
    pub async fn create_client(
        client_repository: &Box<dyn ClientRepository>,
        redirect_repository: &Box<dyn RedirectUriRepository>,
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
            .create(redirect_repository, &client_create, &redirect_create)
            .await
            .map_err(|_| ClientServiceError::NotCreated)
    }

    pub async fn get_client_by_id(
        client_repository: &Box<dyn ClientRepository>,
        id: &str,
    ) -> Result<ClientModel, ClientServiceError> {
        client_repository
            .get_by_id(id)
            .await
            .map_err(|_| ClientServiceError::NotFound)
    }

    pub async fn get_clients_by_user(
        client_repository: &Box<dyn ClientRepository>,
        user_id: &Uuid,
    ) -> Result<Vec<ClientModel>, ClientServiceError> {
        client_repository
            .get_all_by_user_id(user_id)
            .await
            .map_err(|_| ClientServiceError::NotFound)
    }

    pub async fn update_client_by_id(
        client_repository: &Box<dyn ClientRepository>,
        client_id: &str,
        update_client: &ClientUpdateModel,
    ) -> Result<ClientModel, ClientServiceError> {
        client_repository
            .update_by_id(client_id, update_client)
            .await
            .map_err(|_| ClientServiceError::NotUpdated)
    }

    pub async fn delete_client_by_id(
        client_repository: &Box<dyn ClientRepository>,
        client_id: &str,
    ) -> Result<(), ClientServiceError> {
        client_repository
            .delete_by_id(client_id)
            .await
            .map_err(|_| ClientServiceError::BadDelete)
    }

    pub fn generate_random_string() -> String {
        let mut buffer = [0u8; 24];
        let rng = SystemRandom::new();
        rng.fill(&mut buffer).unwrap();
        general_purpose::URL_SAFE_NO_PAD.encode(buffer)
    }
}

pub enum ClientServiceError {
    AlreadyExists,
    NotCreated,
    NotFound,
    NotUpdated,
    BadDelete,
}