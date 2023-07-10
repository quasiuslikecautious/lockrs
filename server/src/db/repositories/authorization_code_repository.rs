use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    db::DbContext,
    oauth2::models::{AuthorizationCodeCreateModel, AuthorizationCodeModel},
};

#[async_trait]
pub trait AuthorizationCodeRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        auth_code_create: &AuthorizationCodeCreateModel,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError>;
    async fn get_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError>;
    async fn get_by_code(
        &self,
        db_context: &Arc<DbContext>,
        code: &str,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError>;
    async fn delete_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError>;
}

pub enum AuthorizationCodeRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    BadDelete,
}
