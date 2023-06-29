use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

use crate::{
    auth::responses::{ClientListResponse, ClientResponse},
    models::{ClientCreateModel, ClientUpdateModel},
    services::{ClientService, ClientServiceError},
    AppState,
};

pub struct ClientController;

#[derive(Deserialize)]
pub struct ClientCreateRequest {
    pub user_id: Uuid,
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
        State(state): State<Arc<AppState>>,
        Path(user_id): Path<Uuid>,
    ) -> Result<ClientListResponse, ClientControllerError> {
        let client_repository = &state.repository_container.as_ref().client_repository;

        let clients = ClientService::get_clients_by_user(client_repository, &user_id)
            .await
            .map_err(|_| ClientControllerError::Internal)?;

        Ok(ClientListResponse {
            clients: clients
                .into_iter()
                .map(|c| ClientResponse {
                    id: c.id,
                    name: c.name,
                    description: c.description,
                    homepage_url: c.homepage_url,
                })
                .collect::<Vec<ClientResponse>>(),
        })
    }

    pub async fn create(
        State(state): State<Arc<AppState>>,
        Json(new_client_request): Json<ClientCreateRequest>,
    ) -> Result<ClientResponse, ClientControllerError> {
        println!("entered route...");

        let new_client = ClientCreateModel {
            user_id: new_client_request.user_id,
            is_public: new_client_request.is_public,
            name: new_client_request.name,
            description: new_client_request.description,
            homepage_url: new_client_request.homepage_url,
            redirect_url: new_client_request.redirect_url,
        };

        let client_repository = &state.repository_container.as_ref().client_repository;
        let redirect_repository = &state.repository_container.as_ref().redirect_repository;

        println!("before create...");

        let client =
            ClientService::create_client(client_repository, redirect_repository, new_client)
                .await
                .map_err(|_| ClientControllerError::Internal)?;

        println!("after create...");

        Ok(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        })
    }

    pub async fn read(
        State(state): State<Arc<AppState>>,
        Path(client_id): Path<String>,
    ) -> Result<ClientResponse, ClientControllerError> {
        let client_repository = &state.repository_container.as_ref().client_repository;

        let client = ClientService::get_client_by_id(client_repository, &client_id)
            .await
            .map_err(|err| match err {
                ClientServiceError::NotFound => ClientControllerError::InvalidClient,
                _ => ClientControllerError::Internal,
            })?;

        Ok(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        })
    }

    pub async fn update(
        State(state): State<Arc<AppState>>,
        Path(client_id): Path<String>,
        Json(update_client_request): Json<ClientUpdateRequest>,
    ) -> Result<ClientResponse, ClientControllerError> {
        let update_client = ClientUpdateModel {
            name: update_client_request.name,
            description: update_client_request.description,
            homepage_url: update_client_request.homepage_url,
        };

        let client_repository = &state.repository_container.as_ref().client_repository;

        let client =
            ClientService::update_client_by_id(client_repository, &client_id, &update_client)
                .await
                .map_err(|err| match err {
                    ClientServiceError::NotFound => ClientControllerError::InvalidClient,
                    _ => ClientControllerError::Internal,
                })?;

        Ok(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        })
    }

    pub async fn delete(
        State(state): State<Arc<AppState>>,
        Path(client_id): Path<String>,
    ) -> Result<StatusCode, ClientControllerError> {
        let client_repository = &state.repository_container.as_ref().client_repository;

        ClientService::delete_client_by_id(client_repository, &client_id)
            .await
            .map_err(|_| ClientControllerError::Internal)?;

        Ok(StatusCode::NO_CONTENT)
    }
}

pub enum ClientControllerError {
    Internal,
    InvalidClient,
}

impl ClientControllerError {
    pub fn error_message(&self) -> &'static str {
        match self {
            Self::Internal => {
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
