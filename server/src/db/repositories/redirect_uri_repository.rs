use async_trait::async_trait;
use url::Url;

use crate::models::{RedirectCreateModel, RedirectModel};

#[async_trait]
pub trait RedirectUriRepository: Send + Sync {
    async fn create(
        &self,
        redirect_create: &RedirectCreateModel,
    ) -> Result<RedirectModel, RedirectUriRepositoryError>;
    async fn get_by_uri(
        &self,
        client_id: &str,
        uri: &Url,
    ) -> Result<RedirectModel, RedirectUriRepositoryError>;
    async fn get_all_by_client_id(
        &self,
        client_id: &str,
    ) -> Result<Vec<RedirectModel>, RedirectUriRepositoryError>;
}

pub enum RedirectUriRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    NoneFound,
}
