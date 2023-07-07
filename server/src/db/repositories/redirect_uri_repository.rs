use std::sync::Arc;

use async_trait::async_trait;
use url::Url;

use crate::{
    db::DbContext,
    models::{RedirectCreateModel, RedirectModel},
};

#[async_trait]
pub trait RedirectUriRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        redirect_create: &RedirectCreateModel,
    ) -> Result<RedirectModel, RedirectUriRepositoryError>;
    async fn get_by_uri(
        &self,
        db_context: &Arc<DbContext>,
        client_id: &str,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectUriRepositoryError>;
    async fn get_all_by_client_id(
        &self,
        db_context: &Arc<DbContext>,
        client_id: &str,
    ) -> Result<Vec<RedirectModel>, RedirectUriRepositoryError>;
}

pub enum RedirectUriRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    NoneFound,
}
