use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    models::{UserCreateModel, UserModel, UserUpdateModel},
    DbContext,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        user_create: &UserCreateModel,
    ) -> Result<UserModel, UserRepositoryError>;
    async fn get_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<UserModel, UserRepositoryError>;
    async fn get_by_email(
        &self,
        db_context: &Arc<DbContext>,
        email: &str,
    ) -> Result<UserModel, UserRepositoryError>;
    async fn update_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
        user_update: &UserUpdateModel,
    ) -> Result<UserModel, UserRepositoryError>;
    async fn delete_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<(), UserRepositoryError>;
}

pub enum UserRepositoryError {
    BadConnection,
    NotCreated,
    AlreadyExists,
    NotFound,
    NotUpdated,
    BadDelete,
}
