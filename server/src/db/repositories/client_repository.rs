use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    db::{repositories::RedirectUriRepository, DbContext},
    models::{ClientModel, ClientUpdateModel, RedirectCreateModel},
};

#[async_trait]
pub trait ClientRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        client_create: &ClientModel,
        redirect_create: &RedirectCreateModel,
    ) -> Result<ClientModel, ClientRepositoryError>;
    async fn get_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<ClientModel, ClientRepositoryError>;
    async fn get_by_credentials(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
        secret: Option<&str>,
    ) -> Result<ClientModel, ClientRepositoryError>;
    async fn get_all_by_user_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<Vec<ClientModel>, ClientRepositoryError>;
    async fn update_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
        client_update: &ClientUpdateModel,
    ) -> Result<ClientModel, ClientRepositoryError>;
    async fn delete_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<(), ClientRepositoryError>;
}

pub enum ClientRepositoryError {
    BadConnection,
    BadTransaction,
    NotCreated,
    NotFound,
    NotUpdated,
    BadDelete,
}
