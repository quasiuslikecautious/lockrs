use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    oauth2::models::{AuthorizationCodeCreateModel, AuthorizationCodeModel},
    repositories::{AuthorizationCodeRepository, AuthorizationCodeRepositoryError},
    DbContext,
};

pub struct PgAuthorizationCodeRepository {
    db_context: Arc<DbContext>,
}

impl PgAuthorizationCodeRepository {
    pub fn new(db_context: &Arc<DbContext>) -> Self {
        Self {
            db_context: Arc::clone(db_context),
        }
    }
}

#[async_trait]
impl AuthorizationCodeRepository for PgAuthorizationCodeRepository {
    async fn create(
        &self,
        _auth_code_create: &AuthorizationCodeCreateModel,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError> {
        todo!();
    }

    async fn get_by_id(
        &self,
        _id: &str,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError> {
        todo!();
    }

    async fn get_by_code(
        &self,
        _code: &str,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError> {
        todo!();
    }

    async fn delete_by_id(
        &self,
        _id: &str,
    ) -> Result<AuthorizationCodeModel, AuthorizationCodeRepositoryError> {
        todo!();
    }
}
