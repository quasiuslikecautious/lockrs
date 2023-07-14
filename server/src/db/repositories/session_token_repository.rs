use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    auth::models::SessionTokenModel,
    db::{repositories::RepositoryError, DbContext},
};

pub type SessionTokenRepositoryArc = Arc<Box<dyn SessionTokenRepository>>;

#[async_trait]
pub trait SessionTokenRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        token: &SessionTokenModel,
    ) -> Result<SessionTokenModel, RepositoryError>;
    async fn get_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<SessionTokenModel, RepositoryError>;
    async fn delete_by_token(
        &self,
        db_context: &Arc<DbContext>,
        token: &str,
    ) -> Result<(), RepositoryError>;
}
