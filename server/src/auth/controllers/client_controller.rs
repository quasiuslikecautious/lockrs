use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

use crate::{
    auth::responses::{ClientListResponse, ClientResponse},
    db::get_connection_from_pool,
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
        Extension(state): Extension<Arc<AppState>>,
        Path(user_id): Path<Uuid>,
    ) -> Result<ClientListResponse, ClientControllerError> {
        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| ClientControllerError::InternalError)?;

        let clients = ClientService::get_clients_by_user(db_connection.as_mut(), &user_id)
            .await
            .map_err(|_| ClientControllerError::InternalError)?;

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
        Extension(state): Extension<Arc<AppState>>,
        Json(new_client_request): Json<ClientCreateRequest>,
    ) -> Result<ClientResponse, ClientControllerError> {
        let new_client = ClientCreateModel {
            user_id: new_client_request.user_id,
            is_public: new_client_request.is_public,
            name: new_client_request.name,
            description: new_client_request.description,
            homepage_url: new_client_request.homepage_url,
            redirect_url: new_client_request.redirect_url,
        };

        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| ClientControllerError::InternalError)?;

        let client = ClientService::create_client(db_connection.as_mut(), new_client)
            .await
            .map_err(|_| ClientControllerError::InternalError)?;

        Ok(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        })
    }

    pub async fn read(
        Extension(state): Extension<Arc<AppState>>,
        Path(client_id): Path<String>,
    ) -> Result<ClientResponse, ClientControllerError> {
        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| ClientControllerError::InternalError)?;

        let client = ClientService::get_client_by_id(db_connection.as_mut(), &client_id)
            .await
            .map_err(|err| match err {
                ClientServiceError::NotFoundError => ClientControllerError::InvalidClient,
                _ => ClientControllerError::InternalError,
            })?;

        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        Ok(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        })
    }

    pub async fn update(
        Extension(state): Extension<Arc<AppState>>,
        Path(client_id): Path<String>,
        Json(update_client_request): Json<ClientUpdateRequest>,
    ) -> Result<ClientResponse, ClientControllerError> {
        let update_client = ClientUpdateModel {
            name: update_client_request.name,
            description: update_client_request.description,
            homepage_url: update_client_request.homepage_url,
        };

        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| ClientControllerError::InternalError)?;

        let client =
            ClientService::update_client_by_id(db_connection.as_mut(), &client_id, update_client)
                .await
                .map_err(|err| match err {
                    ClientServiceError::NotFoundError => ClientControllerError::InvalidClient,
                    _ => ClientControllerError::InternalError,
                })?;

        Ok(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        })
    }

    pub async fn delete(
        Extension(state): Extension<Arc<AppState>>,
        Path(client_id): Path<String>,
    ) -> Result<ClientResponse, ClientControllerError> {
        let mut db_connection = get_connection_from_pool(&state.db_pool)
            .await
            .map_err(|_| ClientControllerError::InternalError)?;

        let client = ClientService::delete_client_by_id(db_connection.as_mut(), &client_id)
            .await
            .map_err(|_| ClientControllerError::InternalError)?;

        Ok(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        })
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
