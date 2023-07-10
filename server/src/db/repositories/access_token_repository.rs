use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    db::DbContext,
    oauth2::models::{AccessTokenCreateModel, AccessTokenModel},
};

#[async_trait]
pub trait AccessTokenRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        token_create: &AccessTokenCreateModel,
    ) -> Result<AccessTokenModel, AccessTokenRepositoryError>;
    async fn get_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<AccessTokenModel, AccessTokenRepositoryError>;
    async fn delete_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<(), AccessTokenRepositoryError>;
}

pub enum AccessTokenRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    BadDelete,
}
