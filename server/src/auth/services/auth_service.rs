use std::sync::Arc;

use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use thiserror::Error;

use crate::{
    auth::{
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

pub struct AuthService;

impl AuthService {
    pub async fn login(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        session_token_repository: &dyn SessionTokenRepository,
        user_auth: &AuthModel,
    ) -> Result<SessionTokenModel, AuthServiceError> {
        let user = UserService::get_user_by_email(db_context, user_repository, &user_auth.email)
            .await
            .map_err(AuthServiceError::from)?;

        Self::verify_password(user_auth.password.as_str(), user.password_hash.as_str())?;

        let session_token = SessionTokenService::create_session_token(
            db_context,
            session_token_repository,
            &user.id,
        )
        .await
        .map_err(AuthServiceError::from)?;

        Ok(session_token)
    }

    pub async fn register_user(
        db_context: &Arc<DbContext>,
        user_repository: &dyn UserRepository,
        register_user: &RegisterModel,
    ) -> Result<UserModel, AuthServiceError> {
        let password_hash = Self::hash_password(register_user.password.as_str())?;

        let create_user = UserCreateModel {
            email: register_user.email.clone(),
            password_hash,
        };

        UserService::create_user(db_context, user_repository, &create_user)
            .await
            .map_err(AuthServiceError::from)
    }

    fn hash_password(password: &str) -> Result<String, AuthServiceError> {
        hash(password, DEFAULT_COST).map_err(AuthServiceError::from)
    }

    fn verify_password(password: &str, hash: &str) -> Result<(), AuthServiceError> {
        let valid_password = verify(password, hash).map_err(AuthServiceError::from)?;

        if !valid_password {
            return Err(AuthServiceError::Credentials(format!(
                "Invalid password supplied"
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("AUTH SERVICE ERROR :: Invalid token :: {0}")]
    Token(String),
    #[error("AUTH SERVICE ERROR :: Invalid credentials :: {0}")]
    Credentials(String),
    #[error("AUTH SERVICE ERROR :: User already exists :: {0}")]
    AlreadyExists(String),
    #[error("AUTH SERVICE ERROR :: User not created :: {0}")]
    NotCreated(String),

    #[error("AUTH SERVICE ERROR :: Internal error :: {0}")]
    InternalError(String),
}

impl From<UserServiceError> for AuthServiceError {
    fn from(err: UserServiceError) -> Self {
        match err {
            UserServiceError::AlreadyExists(msg) => Self::AlreadyExists(msg),
            UserServiceError::NotCreated(msg) => Self::NotCreated(msg),
            UserServiceError::NotFound(msg) => Self::Credentials(msg),
            // QueryFailure::NotUpdated => Self::NotUpdated(msg),
            UserServiceError::InternalError(msg) => Self::InternalError(msg),

            _ => Self::InternalError(format!("TODO Error not implemented")),
        }
    }
}

impl From<SessionTokenServiceError> for AuthServiceError {
    fn from(err: SessionTokenServiceError) -> Self {
        match err {
            SessionTokenServiceError::NotFound(msg) => Self::Token(msg),

            SessionTokenServiceError::NotCreated(msg) => Self::InternalError(msg),
            SessionTokenServiceError::NotDeleted(msg) => Self::InternalError(msg),

            SessionTokenServiceError::InternalError(msg) => Self::InternalError(msg),
        }
    }
}

impl From<BcryptError> for AuthServiceError {
    fn from(err: BcryptError) -> Self {
        Self::InternalError(format!("{:?}", err))
    }
}
