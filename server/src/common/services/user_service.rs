use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::{
    db::{
        repositories::{QueryFailure, RepositoryError, UserRepository},
        DbContext,
    },
    models::{UserCreateModel, UserModel, UserUpdateModel},
};

pub struct UserService {}

impl UserService {
    pub async fn create_user(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        new_user: &UserCreateModel,
    ) -> Result<UserModel, UserServiceError> {
        tracing::trace!(
            method = "create_user",
            user = new_user.email
        );

        user_repository
            .create(db_context, new_user)
            .await
            .map_err(UserServiceError::from)
    }

    pub async fn get_user_by_id(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        id: &Uuid,
    ) -> Result<UserModel, UserServiceError> {
        tracing::trace!(
            method = "get_user_by_id",
            ?id
        );

        user_repository
            .get_by_id(db_context, id)
            .await
            .map_err(UserServiceError::from)
    }

    pub async fn get_user_by_email(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        email: &str,
    ) -> Result<UserModel, UserServiceError> {
        tracing::trace!(
            method = "get_user_by_email",
            email
        );

        user_repository
            .get_by_email(db_context, email)
            .await
            .map_err(UserServiceError::from)
    }

    pub async fn update_user_by_id(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        id: &Uuid,
        update_user: &UserUpdateModel,
    ) -> Result<UserModel, UserServiceError> {
        tracing::trace!(
            method = "update_user_by_id",
            ?id,
            user = ?update_user
        );

        user_repository
            .update_by_id(db_context, id, update_user)
            .await
            .map_err(UserServiceError::from)
    }

    pub async fn delete_user_by_id(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        id: &Uuid,
    ) -> Result<(), UserServiceError> {
        tracing::trace!(
            method = "delete_user_by_id",
            ?id
        );

        user_repository
            .delete_by_id(db_context, id)
            .await
            .map_err(UserServiceError::from)
    }
}

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("USER SERVICE ERROR :: Already Exists :: {0}")]
    AlreadyExists(String),
    #[error("USER SERVICE ERROR :: Not Created :: {0}")]
    NotCreated(String),
    #[error("USER SERVICE ERROR :: Not Found :: {0}")]
    NotFound(String),
    #[error("USER SERVICE ERROR :: Not Updated :: {0}")]
    NotUpdated(String),
    #[error("USER SERVICE ERROR :: Not Deleted :: {0}")]
    NotDeleted(String),

    #[error("USER SERVICE ERROR :: Internal Error :: {0}")]
    InternalError(String),
}

impl From<RepositoryError> for UserServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            RepositoryError::QueryFailed(msg, query_error) => match query_error {
                QueryFailure::AlreadyExists => Self::AlreadyExists(msg),
                QueryFailure::NotCreated => Self::NotCreated(msg),
                QueryFailure::NotFound => Self::NotFound(msg),
                QueryFailure::NotUpdated => Self::NotUpdated(msg),
                QueryFailure::NotDeleted => Self::NotDeleted(msg),
            },

            RepositoryError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}
