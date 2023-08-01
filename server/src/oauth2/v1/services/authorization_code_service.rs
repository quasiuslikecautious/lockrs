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
        tracing::trace!(method = "create");

        todo!();
    }
}

#[derive(Debug, Error)]
pub enum AuthorizationCodeServiceError {
    #[error("AUTHORIZATION CODE SERVICE ERROR :: Not Created")]
    NotCreated,

    #[error("AUTHORIZATION CODE SERVICE ERROR :: Internal Error")]
    InternalError,
}

impl From<RepositoryError> for AuthorizationCodeServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            RepositoryError::QueryFailed(_, query_err) => match query_err {
                QueryFailure::NotCreated => Self::NotCreated,
                _ => Self::InternalError,
            },

            RepositoryError::InternalError(_) => Self::InternalError,
        }
    }
}
