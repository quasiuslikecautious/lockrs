use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    db::{repositories::RepositoryError, DbContext},
    models::{UserModel, UserUpdateModel},
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<UserModel, RepositoryError>;
    async fn get_by_email(
        &self,
        db_context: &Arc<DbContext>,
        email: &str,
    ) -> Result<UserModel, RepositoryError>;
    async fn update_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
        user_update: &UserUpdateModel,
    ) -> Result<UserModel, RepositoryError>;
    async fn delete_by_id(
        &self,
        db_context: &Arc<DbContext>,
        id: &Uuid,
    ) -> Result<(), RepositoryError>;
}
