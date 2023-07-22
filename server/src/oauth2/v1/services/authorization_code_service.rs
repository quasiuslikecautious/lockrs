use std::sync::Arc;

use thiserror::Error;

use crate::{
    db::{
        repositories::{AuthorizationCodeRepository, QueryFailure, RepositoryError},
        DbContext,
    },
    oauth2::v1::models::{AuthorizationCodeCreateModel, AuthorizationCodeModel},
};

pub struct AuthorizationCodeService;

impl AuthorizationCodeService {
    pub async fn create(
        _db_context: &Arc<DbContext>,
        _authorization_code_repository: &dyn AuthorizationCodeRepository,
        _new_code: AuthorizationCodeCreateModel,
    ) -> AuthorizationCodeModel {
        todo!();
    }
}

#[derive(Debug, Error)]
pub enum AuthorizationCodeServiceError {
    #[error("AUTHORIZATION CODE SERVICE ERROR :: Not Created :: {0}")]
    NotCreated(String),

    #[error("AUTHORIZATION CODE SERVICE ERROR :: Internal Error :: {0}")]
    InternalError(String),
}

impl From<RepositoryError> for AuthorizationCodeServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::QueryFailed(msg, query_err) => match query_err {
                QueryFailure::NotCreated => Self::NotCreated(msg),
                _ => Self::InternalError(msg),
            },

            RepositoryError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}
