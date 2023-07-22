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
        refresh_token_repository
            .delete_by_token(db_context, token)
            .await
            .map_err(RefreshTokenServiceError::from)
    }
}

#[derive(Debug, Error)]
pub enum RefreshTokenServiceError {
    #[error("REFRESH TOKEN SERVICE ERROR :: Token not created :: {0}")]
    NotCreated(String),
    #[error("REFRESH TOKEN SERVICE ERROR :: Token not found :: {0}")]
    NotFound(String),
    #[error("REFRESH TOKEN SERVICE ERROR :: Token not updated :: {0}")]
    NotUpdated(String),
    #[error("REFRESH TOKEN SERVICE ERROR :: Token not deleted :: {0}")]
    NotDeleted(String),

    #[error("REFRESH TOKEN SERVICE ERROR :: Internal Error :: {0}")]
    InternalError(String),
}

impl From<RepositoryError> for RefreshTokenServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::QueryFailed(msg, query_err) => match query_err {
                QueryFailure::NotCreated => Self::NotCreated(msg),
                QueryFailure::NotFound => Self::NotFound(msg),
                QueryFailure::NotUpdated => Self::NotUpdated(msg),
                QueryFailure::NotDeleted => Self::NotDeleted(msg),

                _ => Self::InternalError(msg),
            },

            RepositoryError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}
