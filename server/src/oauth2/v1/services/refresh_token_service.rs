use std::sync::Arc;

use thiserror::Error;

use crate::{
    db::{
        repositories::{QueryFailure, RefreshTokenRepository, RepositoryError},
        DbContext,
    },
    oauth2::v1::models::{RefreshTokenCreateModel, RefreshTokenModel},
};

pub struct RefreshTokenService {}

impl RefreshTokenService {
    pub async fn create_token(
        db_context: &Arc<DbContext>,
        refresh_token_repository: &dyn RefreshTokenRepository,
        token_create: &RefreshTokenCreateModel,
    ) -> Result<RefreshTokenModel, RefreshTokenServiceError> {
        tracing::trace!(method = "create_token",);

        refresh_token_repository
            .create(db_context, token_create)
            .await
            .map_err(RefreshTokenServiceError::from)
    }

    pub async fn get_by_token(
        db_context: &Arc<DbContext>,
        refresh_token_repository: &dyn RefreshTokenRepository,
        token: &str,
    ) -> Result<RefreshTokenModel, RefreshTokenServiceError> {
        tracing::trace!(method = "get_by_token",);

        refresh_token_repository
            .get_by_token(db_context, token)
            .await
            .map_err(RefreshTokenServiceError::from)
    }

    pub async fn use_token(
        db_context: &Arc<DbContext>,
        refresh_token_repository: &dyn RefreshTokenRepository,
        token: &str,
    ) -> Result<RefreshTokenModel, RefreshTokenServiceError> {
        tracing::trace!(method = "use_token",);

        refresh_token_repository
            .use_by_token(db_context, token)
            .await
            .map_err(RefreshTokenServiceError::from)
    }

    pub async fn delete_token(
        db_context: &Arc<DbContext>,
        refresh_token_repository: &dyn RefreshTokenRepository,
        token: &str,
    ) -> Result<(), RefreshTokenServiceError> {
        tracing::trace!(method = "delete_token",);

        refresh_token_repository
            .delete_by_token(db_context, token)
            .await
            .map_err(RefreshTokenServiceError::from)
    }
}

#[derive(Debug, Error)]
pub enum RefreshTokenServiceError {
    #[error("REFRESH TOKEN SERVICE ERROR :: Token not created")]
    NotCreated,
    #[error("REFRESH TOKEN SERVICE ERROR :: Token not found")]
    NotFound,
    #[error("REFRESH TOKEN SERVICE ERROR :: Token not updated")]
    NotUpdated,
    #[error("REFRESH TOKEN SERVICE ERROR :: Token not deleted")]
    NotDeleted,

    #[error("REFRESH TOKEN SERVICE ERROR :: Internal Error")]
    InternalError,
}

impl From<RepositoryError> for RefreshTokenServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            RepositoryError::QueryFailed(_msg, query_err) => match query_err {
                QueryFailure::NotCreated => Self::NotCreated,
                QueryFailure::NotFound => Self::NotFound,
                QueryFailure::NotUpdated => Self::NotUpdated,
                QueryFailure::NotDeleted => Self::NotDeleted,

                _ => Self::InternalError,
            },

            RepositoryError::InternalError(_msg) => Self::InternalError,
        }
    }
}
