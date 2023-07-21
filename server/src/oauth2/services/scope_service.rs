use std::sync::Arc;

use thiserror::Error;

use crate::{
    db::{
        repositories::{QueryFailure, RepositoryError, ScopeRepository},
        DbContext,
    },
    oauth2::models::ScopeModel,
};

pub struct ScopeService;

impl ScopeService {
    pub async fn get_from_list(
        db_context: &Arc<DbContext>,
        scope_repository: &dyn ScopeRepository,
        scope: &str,
    ) -> Result<ScopeModel, ScopeServiceError> {
        let scopes_list = scope
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        scope_repository
            .get_from_list(db_context, &scopes_list)
            .await
            .map_err(ScopeServiceError::from)
    }
}

#[derive(Debug, Error)]
pub enum ScopeServiceError {
    #[error("SCOPE SERVICE ERROR :: No valid scopes found :: {0}")]
    InvalidScopes(String),

    #[error("SCOPE SERVICE ERROR :: Internal Error :: {0}")]
    InternalError(String),
}

impl From<RepositoryError> for ScopeServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::QueryFailed(msg, query_err) => match query_err {
                QueryFailure::NotFound => Self::InvalidScopes(msg),

                _ => Self::InternalError(msg),
            },

            RepositoryError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}
