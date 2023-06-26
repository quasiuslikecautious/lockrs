use async_trait::async_trait;
use uuid::Uuid;

use crate::models::{UserCreateModel, UserModel, UserUpdateModel};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user_create: &UserCreateModel)
        -> Result<UserModel, UserRepositoryError>;
    async fn get_by_id(&self, id: &Uuid) -> Result<UserModel, UserRepositoryError>;
    async fn get_by_email(&self, email: &str) -> Result<UserModel, UserRepositoryError>;
    async fn update_by_id(
        &self,
        id: &Uuid,
        user_update: &UserUpdateModel,
    ) -> Result<UserModel, UserRepositoryError>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<(), UserRepositoryError>;
}

pub enum UserRepositoryError {
    BadConnection,
    NotCreated,
    AlreadyExists,
    NotFound,
    NotUpdated,
    BadDelete,
}
