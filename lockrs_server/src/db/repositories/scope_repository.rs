use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    db::{repositories::RepositoryError, DbContext},
    oauth2::v1::models::{ScopeCreateModel, ScopeModel},
};

#[async_trait]
pub trait ScopeRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        scope_create: &ScopeCreateModel,
    ) -> Result<ScopeModel, RepositoryError>;
    async fn get_from_list(
        &self,
        db_context: &Arc<DbContext>,
        scopes_list: &[String],
    ) -> Result<ScopeModel, RepositoryError>;
    async fn delete_by_name(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<(), RepositoryError>;
}
