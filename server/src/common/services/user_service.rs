use uuid::Uuid;

use crate::models::{UserCreateModel, UserModel, UserUpdateModel};
use crate::repositories::UserRepository;

pub struct UserService {}

impl UserService {
    pub async fn create_user(
        user_repository: &dyn UserRepository,
        new_user: &UserCreateModel,
    ) -> Result<UserModel, UserServiceError> {
        user_repository
            .create(new_user)
            .await
            .map_err(|_| UserServiceError::NotCreated)
    }

    pub async fn get_user_by_id(
        user_repository: &dyn UserRepository,
        id: &Uuid,
    ) -> Result<UserModel, UserServiceError> {
        user_repository
            .get_by_id(id)
            .await
            .map_err(|_| UserServiceError::NotFound)
    }

    pub async fn get_user_by_email(
        user_repository: &dyn UserRepository,
        email: &str,
    ) -> Result<UserModel, UserServiceError> {
        user_repository
            .get_by_email(email)
            .await
            .map_err(|_| UserServiceError::NotFound)
    }

    pub async fn update_user_by_id(
        user_repository: &dyn UserRepository,
        id: &Uuid,
        update_user: &UserUpdateModel,
    ) -> Result<UserModel, UserServiceError> {
        user_repository
            .update_by_id(id, update_user)
            .await
            .map_err(|_| UserServiceError::NotUpdated)
    }

    pub async fn delete_user_by_id(
        user_repository: &dyn UserRepository,
        id: &Uuid,
    ) -> Result<(), UserServiceError> {
        user_repository
            .delete_by_id(id)
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
