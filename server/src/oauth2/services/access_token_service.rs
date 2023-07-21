use std::sync::Arc;

use thiserror::Error;

use crate::{
    db::{
        repositories::{AccessTokenRepository, QueryFailure, RepositoryError},
        DbContext,
    },
    oauth2::models::{AccessTokenCreateModel, AccessTokenModel},
};

pub struct AccessTokenService {}

impl AccessTokenService {
    pub async fn create_token(
        db_context: &Arc<DbContext>,
        access_token_repository: &dyn AccessTokenRepository,
        token_create: &AccessTokenCreateModel,
    ) -> Result<AccessTokenModel, AccessTokenServiceError> {
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
        access_token_repository
            .delete_by_token(db_context, token)
            .await
            .map_err(AccessTokenServiceError::from)
    }
}

#[derive(Debug, Error)]
pub enum AccessTokenServiceError {
    #[error("ACCESS TOKEN SERVICE ERROR :: Token not created :: {0}")]
    NotCreated(String),
    #[error("ACCESS TOKEN SERVICE ERROR :: Token not found :: {0}")]
    NotFound(String),
    #[error("ACCESS TOKEN SERVICE ERROR :: Token not deleted :: {0}")]
    NotDeleted(String),

    #[error("ACCESS TOKEN SERVICE ERROR :: Internal Error :: {0}")]
    InternalError(String),
}

impl From<RepositoryError> for AccessTokenServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::QueryFailed(msg, query_err) => match query_err {
                QueryFailure::NotCreated => Self::NotCreated(msg),
                QueryFailure::NotFound => Self::NotFound(msg),
                QueryFailure::NotDeleted => Self::NotDeleted(msg),

                _ => Self::InternalError(msg),
            },

            RepositoryError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}
