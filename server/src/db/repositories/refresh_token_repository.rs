use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    db::DbContext,
    oauth2::models::{RefreshTokenCreateModel, RefreshTokenModel},
};

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        token_create: &RefreshTokenCreateModel,
    ) -> Result<RefreshTokenModel, RefreshTokenRepositoryError>;
    async fn get_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<RefreshTokenModel, RefreshTokenRepositoryError>;
    async fn use_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<RefreshTokenModel, RefreshTokenRepositoryError>;
    async fn delete_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<(), RefreshTokenRepositoryError>;
}

pub enum RefreshTokenRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    NotUpdated,
    BadDelete,
}
