use std::sync::Arc;

use uuid::Uuid;

use crate::{
    db::{repositories::UserRepository, DbContext},
    models::{UserCreateModel, UserModel, UserUpdateModel},
};

pub struct UserService {}

impl UserService {
    pub async fn create_user(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        new_user: &UserCreateModel,
    ) -> Result<UserModel, UserServiceError> {
        user_repository
            .create(db_context, new_user)
            .await
            .map_err(|_| UserServiceError::NotCreated)
    }

    pub async fn get_user_by_id(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        id: &Uuid,
    ) -> Result<UserModel, UserServiceError> {
        user_repository
            .get_by_id(db_context, id)
            .await
            .map_err(|_| UserServiceError::NotFound)
    }

    pub async fn get_user_by_email(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        email: &str,
    ) -> Result<UserModel, UserServiceError> {
        user_repository
            .get_by_email(db_context, email)
            .await
            .map_err(|_| UserServiceError::NotFound)
    }

    pub async fn update_user_by_id(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        id: &Uuid,
        update_user: &UserUpdateModel,
    ) -> Result<UserModel, UserServiceError> {
        user_repository
            .update_by_id(db_context, id, update_user)
            .await
            .map_err(|_| UserServiceError::NotUpdated)
    }

    pub async fn delete_user_by_id(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        id: &Uuid,
    ) -> Result<(), UserServiceError> {
        user_repository
            .delete_by_id(db_context, id)
            .await
            .map_err(|_| UserServiceError::BadDelete)
    }
}

pub enum UserServiceError {
    AlreadyExists,
    NotCreated,
    NotFound,
    NotUpdated,
    BadDelete,
}
