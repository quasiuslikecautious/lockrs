use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    db::{
        repositories::{AuthorizationCodeRepository, RepositoryError},
        DbContext,
    },
    oauth2::v1::models::{AuthorizationCodeCreateModel, AuthorizationCodeModel},
};

pub struct PgAuthorizationCodeRepository;

#[async_trait]
impl AuthorizationCodeRepository for PgAuthorizationCodeRepository {
    async fn create(
        &self,
        _db_context: &Arc<DbContext>,
        _auth_code_create: &AuthorizationCodeCreateModel,
    ) -> Result<AuthorizationCodeModel, RepositoryError> {
        tracing::trace!(
            method = "create"
        );

        todo!();
    }

    async fn get_by_id(
        &self,
        _db_context: &Arc<DbContext>,
        _id: &str,
    ) -> Result<AuthorizationCodeModel, RepositoryError> {
        tracing::trace!(
            method = "get_by_id"
        );

        todo!();
    }

    async fn get_by_code(
        &self,
        _db_context: &Arc<DbContext>,
        _code: &str,
    ) -> Result<AuthorizationCodeModel, RepositoryError> {
        tracing::trace!(
            method = "get_by_code"
        );

        todo!();
    }

    async fn delete_by_id(
        &self,
        _db_context: &Arc<DbContext>,
        _id: &str,
    ) -> Result<AuthorizationCodeModel, RepositoryError> {
        tracing::trace!(
            method = "delete_by_id"
        );

        todo!();
    }
}
