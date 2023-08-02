use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::{
    db::{
        repositories::{ClientRepository, QueryFailure, RepositoryError},
        DbContext,
    },
    models::{ClientModel, ClientUpdateModel},
};

pub struct ClientService;

impl ClientService {
    pub async fn get_client_by_id(
        db_context: &Arc<DbContext>,
        client_repository: &dyn ClientRepository,
        id: &str,
    ) -> Result<ClientModel, ClientServiceError> {
        tracing::trace!(method = "get_client_by_id", id);

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
        tracing::trace!(method = "get_clients_by_user", ?user_id,);

        client_repository
            .get_all_by_user_id(db_context, user_id)
            .await
            .map_err(ClientServiceError::from)
    }

    pub async fn update_client_by_id(
        db_context: &Arc<DbContext>,
        client_repository: &dyn ClientRepository,
        id: &str,
        update_client: &ClientUpdateModel,
    ) -> Result<ClientModel, ClientServiceError> {
        tracing::trace!(
            method = "update_client_by_id",
            id,
            client = ?update_client,
        );

        let client = client_repository
            .update_by_id(db_context, id, update_client)
            .await
            .map_err(ClientServiceError::from)?;

        tracing::info!(
            "Client updated: {{ id: {}, update_model: {:?} }}",
            id,
            update_client,
        );

        Ok(client)
    }

    pub async fn delete_client_by_id(
        db_context: &Arc<DbContext>,
        client_repository: &dyn ClientRepository,
        id: &str,
    ) -> Result<(), ClientServiceError> {
        tracing::trace!(method = "create_client", id);

        client_repository
            .delete_by_id(db_context, id)
            .await
            .map_err(ClientServiceError::from)?;

        tracing::info!("Client deleted: {}", id);

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ClientServiceError {
    #[error("CLIENT SERVICE ERROR :: Already Exists")]
    AlreadyExists,
    #[error("CLIENT SERVICE ERROR :: Not Created")]
    NotCreated,
    #[error("CLIENT SERVICE ERROR :: Not Found")]
    NotFound,
    #[error("CLIENT SERVICE ERROR :: Not Updated")]
    NotUpdated,
    #[error("CLIENT SERVICE ERROR :: Bad Deletion")]
    BadDelete,

    #[error("CLIENT SERVICE ERROR :: Internal Error")]
    InternalError,
}

impl From<RepositoryError> for ClientServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            // BL errors

            // CRUD errors
            RepositoryError::QueryFailed(query_err) => match query_err {
                QueryFailure::AlreadyExists => Self::AlreadyExists,
                QueryFailure::NotCreated => Self::NotCreated,
                QueryFailure::NotFound => Self::NotFound,
                QueryFailure::NotUpdated => Self::NotUpdated,
                QueryFailure::NotDeleted => Self::BadDelete,
            },

            // InternalErrors
            RepositoryError::InternalError => Self::InternalError,
        }
    }
}
