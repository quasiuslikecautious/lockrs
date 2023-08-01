use std::sync::Arc;

use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use thiserror::Error;

use crate::{
    api::v1::{
        models::{AuthModel, RegisterModel, SessionTokenModel},
        services::SessionTokenServiceError,
    },
    db::{
        repositories::{SessionTokenRepository, UserRepository},
        DbContext,
    },
    models::{UserCreateModel, UserModel},
    services::{UserService, UserServiceError},
};

use super::SessionTokenService;

pub struct UserAuthService;

impl UserAuthService {
    pub async fn login(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        session_token_repository: &dyn SessionTokenRepository,
        user_auth: &AuthModel,
    ) -> Result<SessionTokenModel, UserAuthServiceError> {
        tracing::trace!(method = "login",);

        let user = UserService::get_user_by_email(db_context, user_repository, &user_auth.email)
            .await
            .map_err(UserAuthServiceError::from)?;

        Self::verify_password(user_auth.password.as_str(), user.password_hash.as_str())?;

        let session_token = SessionTokenService::create_session_token(
            db_context,
            session_token_repository,
            &user.id,
        )
        .await
        .map_err(UserAuthServiceError::from)?;

        tracing::info!(
            "User successfully authenticated with ID: {}",
            session_token.user_id.to_string()
        );

        Ok(session_token)
    }

    pub async fn register_user(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        register_user: &RegisterModel,
    ) -> Result<UserModel, UserAuthServiceError> {
        tracing::trace!(method = "register_user",);

        let password_hash = Self::hash_password(register_user.password.as_str())?;

        let create_user = UserCreateModel {
            email: register_user.email.clone(),
            password_hash,
        };

        let user = UserService::create_user(db_context, user_repository, &create_user)
            .await
            .map_err(UserAuthServiceError::from)?;

        tracing::info!(
            "New user successfully registered with ID: {}",
            user.id.to_string()
        );

        Ok(user)
    }

    fn hash_password(password: &str) -> Result<String, UserAuthServiceError> {
        hash(password, DEFAULT_COST).map_err(UserAuthServiceError::from)
    }

    fn verify_password(password: &str, hash: &str) -> Result<(), UserAuthServiceError> {
        let valid_password = verify(password, hash).map_err(UserAuthServiceError::from)?;

        if !valid_password {
            let msg = "Invalid password supplied";
            tracing::error!(error = msg);
            return Err(UserAuthServiceError::Credentials);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum UserAuthServiceError {
    #[error("AUTH SERVICE ERROR :: Invalid token")]
    Token,
    #[error("AUTH SERVICE ERROR :: Invalid credentials")]
    Credentials,
    #[error("AUTH SERVICE ERROR :: User already exists")]
    AlreadyExists,
    #[error("AUTH SERVICE ERROR :: User not created")]
    NotCreated,

    #[error("AUTH SERVICE ERROR :: Internal error")]
    InternalError,
}

impl From<UserServiceError> for UserAuthServiceError {
    fn from(err: UserServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            UserServiceError::AlreadyExists => Self::AlreadyExists,
            UserServiceError::NotCreated => Self::NotCreated,
            UserServiceError::NotFound => Self::Credentials,
            // QueryFailure::NotUpdated => Self::NotUpdated(msg),
            UserServiceError::InternalError => Self::InternalError,

            _ => Self::InternalError,
        }
    }
}

impl From<SessionTokenServiceError> for UserAuthServiceError {
    fn from(err: SessionTokenServiceError) -> Self {
        tracing::error!(error = %err);

        match err {
            SessionTokenServiceError::NotFound => Self::Token,

            SessionTokenServiceError::NotCreated => Self::InternalError,
            SessionTokenServiceError::NotDeleted => Self::InternalError,

            SessionTokenServiceError::InternalError => Self::InternalError,
        }
    }
}

impl From<BcryptError> for UserAuthServiceError {
    fn from(err: BcryptError) -> Self {
        tracing::error!(error = ?err);

        Self::InternalError
    }
}
