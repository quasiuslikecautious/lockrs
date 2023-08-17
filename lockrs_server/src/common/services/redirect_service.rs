use std::sync::Arc;

use thiserror::Error;
use url::Url;
use uuid::Uuid;

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
        new_redirect: &RedirectCreateModel,
    ) -> Result<RedirectModel, RedirectServiceError> {
        tracing::trace!(method = "create_redirect", ?new_redirect,);

        let redirect_create =
            RedirectCreateModel::new(new_redirect.client_id.as_str(), &new_redirect.uri);

        let redirect = redirect_repository
            .create(db_context, &redirect_create)
            .await
            .map_err(RedirectServiceError::from)?;

        tracing::info!("Redirect Uri created: {:?}", redirect);

        Ok(redirect)
    }

    pub async fn get_redirect_by_id(
        db_context: &Arc<DbContext>,
        redirect_repository: &dyn RedirectUriRepository,
        id: &Uuid,
    ) -> Result<RedirectModel, RedirectServiceError> {
        tracing::trace!(method = "get_redirect_by_id", ?id);

        redirect_repository
            .get_by_id(db_context, id)
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

        let redirect = redirect_repository
            .get_by_uri(db_context, client_id, uri)
            .await
            .map_err(RedirectServiceError::from)?;

        tracing::debug!(
            "Redirect verified: {{ id: {}, uri: {}, client_id: {} }}",
            redirect.id,
            uri.to_string(),
            client_id
        );

        Ok(redirect)
    }

    pub async fn get_user_id_from_redirect_id(
        db_context: &Arc<DbContext>,
        redirect_repository: &dyn RedirectUriRepository,
        redirect_id: &Uuid,
    ) -> Result<Uuid, RedirectServiceError> {
        tracing::trace!(method = "get_user_id_from_redirect_id", ?redirect_id);

        redirect_repository
            .get_user_id(db_context, redirect_id)
            .await
            .map_err(RedirectServiceError::from)
    }

    pub async fn get_redirects_from_client(
        db_context: &Arc<DbContext>,
        redirect_repository: &dyn RedirectUriRepository,
        client_id: &str,
    ) -> Result<Vec<RedirectModel>, RedirectServiceError> {
        tracing::trace!(method = "get_redirects_from_client", client_id);

        redirect_repository
            .get_all_by_client_id(db_context, client_id)
            .await
            .map_err(RedirectServiceError::from)
    }

    pub async fn delete_redirect_by_id(
        db_context: &Arc<DbContext>,
        redirect_repository: &dyn RedirectUriRepository,
        id: &Uuid,
    ) -> Result<(), RedirectServiceError> {
        tracing::trace!(method = "delete_redirect_by_id", ?id);

        // verify client has at least one redirect uri
        let redirect = redirect_repository
            .get_by_id(db_context, id)
            .await
            .map_err(RedirectServiceError::from)?;

        let client_redirects = redirect_repository
            .get_all_by_client_id(db_context, redirect.client_id.as_str())
            .await
            .map_err(RedirectServiceError::from)?;

        if !client_redirects.is_empty() {
            tracing::error!(error = "Client must have at least one redirect uri assigned");
            return Err(RedirectServiceError::TooFewRedirects);
        }

        redirect_repository
            .delete_by_id(db_context, id)
            .await
            .map_err(RedirectServiceError::from)?;

        tracing::info!("Redirect deleted: {:?}", id);

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RedirectServiceError {
    #[error("REDIRECT SERVICE ERROR :: Redirect already exists")]
    AlreadyExists,
    #[error("REDIRECT SERVICE ERROR :: Redirect not created")]
    NotCreated,
    #[error("REDIRECT SERVICE ERROR :: Redirect not found")]
    NotFound,
    #[error("REDIRECT SERVICE ERROR :: Minimum client redirect violation")]
    TooFewRedirects,
    #[error("REDIRECT SERVICE ERROR :: Failed to delete redirect")]
    NotDeleted,

    #[error("REDIRECT SERVICE ERROR :: Internal error")]
    InternalError,
}

impl From<RepositoryError> for RedirectServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            RepositoryError::QueryFailed(query_err) => match query_err {
                QueryFailure::AlreadyExists => Self::AlreadyExists,
                QueryFailure::NotCreated => Self::NotCreated,
                QueryFailure::NotFound => Self::NotFound,
                QueryFailure::NotDeleted => Self::NotDeleted,
                _ => Self::InternalError,
            },

            RepositoryError::InternalError => Self::InternalError,
        }
    }
}
