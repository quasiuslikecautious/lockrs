use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    db::{repositories::RepositoryError, DbContext},
    models::{ClientModel, ClientUpdateModel},
};

#[async_trait]
pub trait ClientRepository: Send + Sync {
    async fn get_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<ClientModel, RepositoryError>;
    async fn get_all_by_user_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<Vec<ClientModel>, RepositoryError>;
    async fn update_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
        client_update: &ClientUpdateModel,
    ) -> Result<ClientModel, RepositoryError>;
    async fn delete_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<(), RepositoryError>;
}
