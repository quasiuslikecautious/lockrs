use async_trait::async_trait;

use crate::oauth2::models::{DeviceAuthorizationCreateModel, DeviceAuthorizationModel};

#[async_trait]
pub trait DeviceAuthorizationRepository: Send + Sync {
    async fn create(
        &self,
        device_authorization_create: &DeviceAuthorizationCreateModel,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationRepositoryError>;
    async fn get_by_user_code(
        &self,
        code: &str,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationRepositoryError>;
    async fn get_by_device_code(
        &self,
        code: &str,
    ) -> Result<DeviceAuthorizationModel, DeviceAuthorizationRepositoryError>;
    async fn delete_by_device_code(
        &self,
        id: &str,
    ) -> Result<(), DeviceAuthorizationRepositoryError>;
}

pub enum DeviceAuthorizationRepositoryError {
    BadConnection,
    NotCreated,
    NotFound,
    BadDelete,
}
