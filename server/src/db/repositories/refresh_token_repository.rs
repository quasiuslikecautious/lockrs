use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    db::{repositories::RepositoryError, DbContext},
    oauth2::v1::models::{RefreshTokenCreateModel, RefreshTokenModel},
};

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        token_create: &RefreshTokenCreateModel,
    ) -> Result<RefreshTokenModel, RepositoryError>;
    async fn get_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<RefreshTokenModel, RepositoryError>;
    async fn use_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<RefreshTokenModel, RepositoryError>;
    async fn delete_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<(), RepositoryError>;
}
