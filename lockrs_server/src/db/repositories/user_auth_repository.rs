use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::v1::models::{UserAuthModel, UserRegisterModel},
    db::{repositories::RepositoryError, DbContext},
};

#[async_trait]
pub trait UserAuthRepository: Send + Sync {
    async fn create(
        &self,
        db_context: &Arc<DbContext>,
        user_create: &UserRegisterModel,
    ) -> Result<UserAuthModel, RepositoryError>;

    async fn get_by_email(
        &self,
        db_context: &Arc<DbContext>,
        email: &str,
    ) -> Result<UserAuthModel, RepositoryError>;
}
