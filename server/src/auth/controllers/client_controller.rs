use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

use crate::{
    auth::responses::{ClientListResponse, ClientResponse},
    models::{ClientCreateModel, ClientUpdateModel},
    services::{ClientService, ClientServiceError},
};

pub struct ClientController;

#[derive(Deserialize)]
pub struct ClientCreateRequest {
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub homepage_url: Url,
    pub redirect_url: Url,
}

#[derive(Deserialize)]
pub struct ClientUpdateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
}

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
                .collect::<Vec<ClientResponse>>(),
        }))
    }

    pub async fn create(
        Path(user_id): Path<Uuid>,
        Json(new_client_request): Json<ClientCreateRequest>,
    ) -> Result<Json<ClientResponse>, ClientControllerError> {
        let new_client = ClientCreateModel {
            is_public: new_client_request.is_public,
            user_id,
            name: new_client_request.name,
            description: new_client_request.description,
            homepage_url: new_client_request.homepage_url,
            redirect_url: new_client_request.redirect_url,
        };

        let client = ClientService::create_client(new_client)
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
        let client = ClientService::get_client_by_id(&client_id).map_err(|err| match err {
            ClientServiceError::NotFoundError => ClientControllerError::InvalidClient,
            _ => ClientControllerError::InternalError,
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
        Json(update_client_request): Json<ClientUpdateRequest>,
    ) -> Result<Json<ClientResponse>, ClientControllerError> {
        let update_client = ClientUpdateModel {
            name: update_client_request.name,
            description: update_client_request.description,
            homepage_url: update_client_request.homepage_url,
        };

        let client = ClientService::update_client_by_id(&client_id, update_client).map_err(
            |err| match err {
                ClientServiceError::NotFoundError => ClientControllerError::InvalidClient,
                _ => ClientControllerError::InternalError,
            },
        )?;

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
            homepage_url: client.homepage_url,
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
            Self::InternalError => {
                "An error has occurred while processing your request. Please try again later."
            }
            Self::InvalidClient => "The provided client is invalid.",
        }
    }
}

impl IntoResponse for ClientControllerError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.error_message()).into_response()
    }
}
