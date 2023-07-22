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
        client_repository
            .get_by_credentials(db_context, id, secret)
            .await
            .map_err(ClientAuthServiceError::from)
    }
}

#[derive(Debug, Error)]
pub enum ClientAuthServiceError {
    #[error("CLIENT AUTH SERVICE ERROR :: Not found :: {0}")]
    NotFound(String),

    #[error("CLIENT AUTH SERVICE ERROR :: Internal Error :: {0}")]
    InternalError(String),
}

impl From<RepositoryError> for ClientAuthServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::QueryFailed(msg, query_err) => match query_err {
                QueryFailure::NotFound => Self::NotFound(msg),

                _ => Self::InternalError(msg),
            },

            RepositoryError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}
