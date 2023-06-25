use async_trait::async_trait;

use crate::oauth2::models::{AuthorizationCodeCreateModel, AuthorizationCodeModel};

#[async_trait]
pub trait AuthorizationCodeRepository: Send + Sync {
    async fn create(
        &self,
        auth_code_create: &AuthorizationCodeCreateModel,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError>;
    async fn get_by_id(
        &self,
        id: &str,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError>;
    async fn get_by_code(
        &self,
        code: &str,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError>;
    async fn delete_by_id(
        &self,
        id: &str,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError>;
}

pub enum AuthorizationCodeRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    BadDelete,
}
