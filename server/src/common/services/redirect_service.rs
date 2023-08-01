use std::sync::Arc;

use thiserror::Error;
use url::Url;

use crate::{
    db::{
        repositories::{QueryFailure, RedirectUriRepository, RepositoryError},
        DbContext,
    },
    models::{RedirectCreateModel, RedirectModel},
};

pub struct RedirectService;

impl RedirectService {
    pub async fn create_redirect(
        db_context: &Arc<DbContext>,
        redirect_repository: &dyn RedirectUriRepository,
        client_id: &str,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectServiceError> {
        tracing::trace!(
            method = "create_redirect",
            client_id,
            %uri
        );

        let redirect_create = RedirectCreateModel {
            client_id: client_id.to_string(),
            uri: uri.clone(),
        };

        redirect_repository
            .create(db_context, &redirect_create)
            .await
            .map_err(RedirectServiceError::from)
    }

    pub async fn verify_redirect(
        db_context: &Arc<DbContext>,
        redirect_repository: &dyn RedirectUriRepository,
        client_id: &str,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectServiceError> {
        tracing::trace!(
            method = "verify_redirect",
            client_id,
            %uri
        );

        redirect_repository
            .get_by_uri(db_context, client_id, uri)
            .await
            .map_err(RedirectServiceError::from)
    }

    pub async fn get_redirects_from_client(
        db_context: &Arc<DbContext>,
        redirect_repository: &dyn RedirectUriRepository,
        client_id: &str,
    ) -> Result<Vec<RedirectModel>, RedirectServiceError> {
        tracing::trace!(method = "get_redirects_from_client", client_id,);

        redirect_repository
            .get_all_by_client_id(db_context, client_id)
            .await
            .map_err(RedirectServiceError::from)
    }
}

#[derive(Debug, Error)]
pub enum RedirectServiceError {
    #[error("REDIRECT SERVICE ERROR :: Redirect not created")]
    NotCreated,
    #[error("REDIRECT SERVICE ERROR :: Redirect not found")]
    NotFound,

    #[error("REDIRECT SERVICE ERROR :: Internal error")]
    InternalError,
}

impl From<RepositoryError> for RedirectServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            RepositoryError::QueryFailed(query_err) => match query_err {
                QueryFailure::NotCreated => Self::NotCreated,
                QueryFailure::NotFound => Self::NotFound,
                _ => Self::InternalError,
            },

            RepositoryError::InternalError => Self::InternalError,
        }
    }
}
