use std::sync::Arc;

use crate::{oauth2::models::ScopeModel, repositories::ScopeRepository, DbContext};

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
            .map_err(|_| ScopeServiceError::InvalidScopes)
    }
}

pub enum ScopeServiceError {
    InvalidScopes,
}
