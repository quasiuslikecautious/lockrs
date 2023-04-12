use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse, Json,
};
use uuid::Uuid;

use crate::{
    auth::responses::{ClientResponse, ClientListResponse},
    models::{NewClientModel, UpdateClientModel}, 
    services::{ClientService, ClientServiceError}, 
};

pub struct ClientController;

impl ClientController {
    pub async fn read_all(
        Path(user_id): Path<Uuid>,
    ) -> Result<Json<ClientListResponse>, ClientControllerError> {
        let clients = ClientService::get_clients_by_user(&user_id)
            .map_err(|_| ClientControllerError::InternalError)?;

        Ok(Json(ClientListResponse {
            clients: clients
                .into_iter()
                .map(|c| ClientResponse { 
                    id: c.id, 
                    name: c.name,
                    description: c.description,
                    homepage_url: c.homepage_url,
                })
                .collect::<Vec<ClientResponse>>()
        }))
    }

    pub async fn create(
        Path(user_id): Path<Uuid>,
        Json(new_client): Json<NewClientModel>,
    ) -> Result<Json<ClientResponse>, ClientControllerError> {
        let client = ClientService::create_client(new_client, &user_id)
            .map_err(|_| ClientControllerError::InternalError)?;
        
        Ok(Json(ClientResponse { 
            id: client.id, 
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        }))
    }

    pub async fn read(
        Path((_user_id, client_id)): Path<(Uuid, String)>,
    ) -> Result<Json<ClientResponse>, ClientControllerError> {
        let client = ClientService::get_client_by_id(&client_id)
            .map_err(|err| {
                match err {
                    ClientServiceError::NotFoundError => ClientControllerError::InvalidClient,
                    _ => ClientControllerError::InternalError,
                } 
            })?;

        Ok(Json(ClientResponse { 
            id: client.id, 
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        }))
    }

    pub async fn update(
        Path((_user_id, client_id)): Path<(Uuid, String)>,
        Json(update_client): Json<UpdateClientModel>,
    ) -> Result<Json<ClientResponse>, ClientControllerError> {
        let client = ClientService::update_client_by_id(&client_id, update_client)
            .map_err(|err| {
                match err {
                    ClientServiceError::NotFoundError => ClientControllerError::InvalidClient,
                    _ => ClientControllerError::InternalError,
                }
            })?;

        Ok(Json(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        }))
    }

    pub async fn delete(
        Path((_user_id, client_id)): Path<(Uuid, String)>,
    ) -> Result<Json<ClientResponse>, ClientControllerError> {
        let client = ClientService::delete_client_by_id(&client_id)
            .map_err(|_| ClientControllerError::InternalError)?;

        Ok(Json(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url
        }))
    }
}

pub enum ClientControllerError {
    InternalError,
    InvalidClient,
}

impl ClientControllerError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InternalError => "An error has occurred while processing your request. Please try again later.",
            Self::InvalidClient => "The provided client is invalid.",
        }
    }
}

impl IntoResponse for ClientControllerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}

