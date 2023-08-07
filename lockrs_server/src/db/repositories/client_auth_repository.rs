use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    db::{repositories::RepositoryError, DbContext},
    models::{ClientAuthModel, RedirectCreateModel},
};

#[async_trait]
pub trait ClientAuthRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        client_create: &ClientAuthModel,
        redirect_create: &RedirectCreateModel,
    ) -> Result<ClientAuthModel, RepositoryError>;
    async fn get_by_credentials(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
        secret: Option<&str>,
    ) -> Result<ClientAuthModel, RepositoryError>;
}
