use async_trait::async_trait;

use crate::oauth2::models::{ScopeCreateModel, ScopeModel};

#[async_trait]
pub trait ScopeRepository: Send + Sync {
    async fn create(
        &self,
        scope_create: &ScopeCreateModel,
    ) -> Result<ScopeModel, ScopeRepositoryError>;
    async fn get_from_list(
        &self,
        scopes_list: &Vec<String>,
    ) -> Result<ScopeModel, ScopeRepositoryError>;
    async fn delete_by_name(&self, id: &str) -> Result<(), ScopeRepositoryError>;
}

pub enum ScopeRepositoryError {
    BadConnection,
    NoneFound,
    BadDelete,
}
