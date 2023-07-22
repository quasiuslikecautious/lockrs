use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    db::{repositories::RepositoryError, DbContext},
    oauth2::v1::models::{AuthorizationCodeCreateModel, AuthorizationCodeModel},
};

#[async_trait]
pub trait AuthorizationCodeRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        auth_code_create: &AuthorizationCodeCreateModel,
    ) -> Result<AuthorizationCodeModel, RepositoryError>;
    async fn get_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<AuthorizationCodeModel, RepositoryError>;
    async fn get_by_code(
        &self,
        db_context: &Arc<DbContext>,
        code: &str,
    ) -> Result<AuthorizationCodeModel, RepositoryError>;
    async fn delete_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<AuthorizationCodeModel, RepositoryError>;
}
