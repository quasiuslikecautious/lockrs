use async_trait::async_trait;
use uuid::Uuid;

use crate::models::{ClientModel, ClientUpdateModel, RedirectCreateModel};
use crate::repositories::RedirectUriRepository;

#[async_trait]
pub trait ClientRepository: Send + Sync {
    async fn create(
        &self,
        redirect_repo: &dyn RedirectUriRepository,
        client_create: &ClientModel,
        redirect_create: &RedirectCreateModel,
    ) -> Result<ClientModel, ClientRepositoryError>;
    async fn get_by_id(&self, id: &str) -> Result<ClientModel, ClientRepositoryError>;
    async fn get_by_credentials(
        &self,
        id: &str,
        secret: &Option<String>,
    ) -> Result<ClientModel, ClientRepositoryError>;
    async fn get_all_by_user_id(
        &self,
        id: &Uuid,
    ) -> Result<Vec<ClientModel>, ClientRepositoryError>;
    async fn update_by_id(
        &self,
        id: &str,
        client_update: &ClientUpdateModel,
    ) -> Result<ClientModel, ClientRepositoryError>;
    async fn delete_by_id(&self, id: &str) -> Result<(), ClientRepositoryError>;
}

pub enum ClientRepositoryError {
    BadConnection,
    BadTransaction,
    NotCreated,
    NotFound,
    NotUpdated,
    BadDelete,
}
