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

        access_token_repository
            .create(db_context, token_create)
            .await
            .map_err(AccessTokenServiceError::from)
    }

    pub async fn verify_token(
        db_context: &Arc<DbContext>,
        access_token_repository: &dyn AccessTokenRepository,
        token: &str,
    ) -> Result<AccessTokenModel, AccessTokenServiceError> {
        tracing::trace!(method = "verify_token",);

        access_token_repository
            .get_by_token(db_context, token)
            .await
            .map_err(AccessTokenServiceError::from)
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
            .map_err(AccessTokenServiceError::from)
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
            RepositoryError::QueryFailed(_msg, query_err) => match query_err {
                QueryFailure::NotCreated => Self::NotCreated,
                QueryFailure::NotFound => Self::NotFound,
                QueryFailure::NotDeleted => Self::NotDeleted,

                _ => Self::InternalError,
            },

            RepositoryError::InternalError(_msg) => Self::InternalError,
        }
    }
}
