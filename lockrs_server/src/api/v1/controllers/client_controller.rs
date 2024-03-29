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
    api::v1::responses::{ClientListResponse, ClientResponse},
    models::ClientUpdateModel,
    services::{ClientService, ClientServiceError},
    AppState,
};

pub struct ClientController;

#[derive(Debug, Deserialize)]
pub struct ClientCreateRequest {
    pub user_id: Uuid,
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub homepage_url: Url,
    pub redirect_url: Url,
}

#[derive(Debug, Deserialize)]
pub struct ClientUpdateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
}

impl ClientController {
    pub async fn read_all(
        State(state): State<AppState>,
        Path(user_id): Path<Uuid>,
    ) -> Result<ClientListResponse, ClientControllerError> {
        tracing::trace!(method = "read_all", user_id = user_id.to_string());

        let db_context = &state.db_context;
        let client_repository = &*state.repository_container.as_ref().client_repository;

        let clients = ClientService::get_clients_by_user(db_context, client_repository, &user_id)
            .await
            .map_err(ClientControllerError::from)?;

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

    pub async fn read(
        State(state): State<AppState>,
        Path(client_id): Path<String>,
    ) -> Result<ClientResponse, ClientControllerError> {
        tracing::trace!(method = "read", user_id = client_id);

        let db_context = &state.db_context;
        let client_repository = &*state.repository_container.as_ref().client_repository;

        let client =
            ClientService::get_client_by_id(db_context, client_repository, client_id.as_str())
                .await
                .map_err(ClientControllerError::from)?;

        Ok(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        })
    }

    pub async fn update(
        State(state): State<AppState>,
        Path(client_id): Path<String>,
        Json(update_client_request): Json<ClientUpdateRequest>,
    ) -> Result<ClientResponse, ClientControllerError> {
        tracing::trace!(
            method = "update",
            params = ?update_client_request
        );

        let update_client = ClientUpdateModel::new(
            update_client_request.name.as_deref(),
            update_client_request.description.as_deref(),
            update_client_request.homepage_url.as_deref(),
        );

        let db_context = &state.db_context;
        let client_repository = &*state.repository_container.as_ref().client_repository;

        let client = ClientService::update_client_by_id(
            db_context,
            client_repository,
            &client_id,
            &update_client,
        )
        .await
        .map_err(ClientControllerError::from)?;

        Ok(ClientResponse {
            id: client.id,
            name: client.name,
            description: client.description,
            homepage_url: client.homepage_url,
        })
    }

    pub async fn delete(
        State(state): State<AppState>,
        Path(client_id): Path<String>,
    ) -> Result<StatusCode, ClientControllerError> {
        tracing::trace!(method = "delete", user_id = client_id);

        let db_context = &state.db_context;
        let client_repository = &*state.repository_container.as_ref().client_repository;

        ClientService::delete_client_by_id(db_context, client_repository, client_id.as_str())
            .await
            .map_err(ClientControllerError::from)?;

        Ok(StatusCode::NO_CONTENT)
    }
}

pub enum ClientControllerError {
    InvalidClient,

    BadRequest,
    Internal,
}

impl ClientControllerError {
    pub fn error_code(&self) -> StatusCode {
        match self {
            Self::InvalidClient => StatusCode::BAD_REQUEST,

            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    pub fn error_message(&self) -> &'static str {
        match self {
            Self::InvalidClient => "The provided client is invalid.",

            Self::BadRequest => "Unable to perform the requested operation.",
            Self::Internal => {
                "An error has occurred while processing your request. Please try again later."
            }
        }
    }
}

impl From<ClientServiceError> for ClientControllerError {
    fn from(err: ClientServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            ClientServiceError::NotFound => Self::InvalidClient,
            ClientServiceError::InternalError => Self::Internal,

            _ => Self::BadRequest,
        }
    }
}

impl IntoResponse for ClientControllerError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code(), self.error_message()).into_response()
    }
}
