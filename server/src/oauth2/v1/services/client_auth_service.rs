use std::sync::Arc;

use thiserror::Error;

use crate::{
    db::{
        repositories::{ClientRepository, QueryFailure, RepositoryError},
        DbContext,
    },
    models::ClientModel,
};

pub struct ClientAuthService;

impl ClientAuthService {
    pub async fn verify_credentials(
        db_context: &Arc<DbContext>,
        client_repository: &dyn ClientRepository,
        id: &str,
        secret: Option<&str>,
    ) -> Result<ClientModel, ClientAuthServiceError> {
        tracing::trace!(method = "verify_credentials", id);

        let client = client_repository
            .get_by_credentials(db_context, id, secret)
            .await
            .map_err(ClientAuthServiceError::from)?;

        tracing::info!("Client authenticated with ID: {}", id);

        Ok(client)
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
