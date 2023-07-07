use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    oauth2::models::{DeviceAuthorizationCreateModel, DeviceAuthorizationModel},
    DbContext,
};

#[async_trait]
pub trait DeviceAuthorizationRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        device_authorization_create: &DeviceAuthorizationCreateModel,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationRepositoryError>;
    async fn get_by_user_code(
        &self,
        db_context: &Arc<DbContext>,
        code: &str,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationRepositoryError>;
    async fn get_by_device_code(
        &self,
        db_context: &Arc<DbContext>,
        code: &str,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationRepositoryError>;
    async fn delete_by_device_code(
        &self,
        db_context: &Arc<DbContext>,
        id: &str,
    ) -> Result<(), DeviceAuthorizationRepositoryError>;
}

pub enum DeviceAuthorizationRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    BadDelete,
}
