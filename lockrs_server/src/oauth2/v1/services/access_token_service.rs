use std::sync::Arc;

use thiserror::Error;

use crate::{
    db::{
        repositories::{AccessTokenRepository, QueryFailure, RepositoryError},
        DbContext,
    },
    oauth2::v1::models::{AccessTokenCreateModel, AccessTokenModel},
};

pub struct AccessTokenService {}

impl AccessTokenService {
    pub async fn create_token(
        db_context: &Arc<DbContext>,
        access_token_repository: &dyn AccessTokenRepository,
        token_create: &AccessTokenCreateModel,
    ) -> Result<AccessTokenModel, AccessTokenServiceError> {
        tracing::trace!(method = "create_token",);

        let token = access_token_repository
            .create(db_context, token_create)
            .await
            .map_err(AccessTokenServiceError::from)?;

        tracing::info!(
            "Access Token created: {{ client_id: {}, expires_at: {}, scopes: {:?} }}",
            &token.client_id,
            &token.expires_at.timestamp(),
            &token.scopes
        );

        Ok(token)
    }

    pub async fn verify_token(
        db_context: &Arc<DbContext>,
        access_token_repository: &dyn AccessTokenRepository,
        token: &str,
    ) -> Result<AccessTokenModel, AccessTokenServiceError> {
        tracing::trace!(method = "verify_token",);

        let access_token = access_token_repository
            .get_by_token(db_context, token)
            .await
            .map_err(AccessTokenServiceError::from)?;

        tracing::info!(
            "Access Token verified: {{ client_id: {}, expires_at: {}, scopes: {:?} }}",
            &access_token.client_id,
            &access_token.expires_at.timestamp(),
            &access_token.scopes
        );

        Ok(access_token)
    }

    pub async fn delete_token(
        db_context: &Arc<DbContext>,
        access_token_repository: &dyn AccessTokenRepository,
        token: &str,
    ) -> Result<(), AccessTokenServiceError> {
        tracing::trace!(method = "delete_token",);

        access_token_repository
            .delete_by_token(db_context, token)
            .await
            .map_err(AccessTokenServiceError::from)?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum AccessTokenServiceError {
    #[error("ACCESS TOKEN SERVICE ERROR :: Token not created")]
    NotCreated,
    #[error("ACCESS TOKEN SERVICE ERROR :: Token not found")]
    NotFound,
    #[error("ACCESS TOKEN SERVICE ERROR :: Token not deleted")]
    NotDeleted,

    #[error("ACCESS TOKEN SERVICE ERROR :: Internal Error")]
    InternalError,
}

impl From<RepositoryError> for AccessTokenServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            RepositoryError::QueryFailed(query_err) => match query_err {
                QueryFailure::NotCreated => Self::NotCreated,
                QueryFailure::NotFound => Self::NotFound,
                QueryFailure::NotDeleted => Self::NotDeleted,

                _ => Self::InternalError,
            },

            RepositoryError::InternalError => Self::InternalError,
        }
    }
}
