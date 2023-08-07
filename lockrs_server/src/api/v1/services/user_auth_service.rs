use std::sync::Arc;

use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use thiserror::Error;

use crate::{
    api::v1::{
        mappers::UserAuthMapper,
        models::{SessionTokenModel, UserLoginCredentials, UserRegisterModel, UserRegistration},
        services::SessionTokenServiceError,
    },
    db::{
        repositories::{QueryFailure, RepositoryError, SessionTokenRepository, UserAuthRepository},
        DbContext,
    },
    models::UserModel,
};

use super::SessionTokenService;

pub struct UserAuthService;

impl UserAuthService {
    pub async fn register_user(
        db_context: &Arc<DbContext>,
        user_auth_repository: &dyn UserAuthRepository,
        register_user: &UserRegistration,
    ) -> Result<UserModel, UserAuthServiceError> {
        tracing::trace!(method = "register_user",);

        let password_hash = Self::hash_password(register_user.password.as_str())?;

        let create_user = UserRegisterModel::new(
            register_user.email.as_str(),
            password_hash.as_str(),
        );

        let user = user_auth_repository
            .create(db_context, &create_user)
            .await
            .map_err(UserAuthServiceError::from)?;

        tracing::info!(
            "User created: {{ id: {}, email: {} }}",
            user.id.to_string(),
            user.email,
        );

        Ok(UserAuthMapper::into_user(user))
    }

    pub async fn login(
        db_context: &Arc<DbContext>,
        user_auth_repository: &dyn UserAuthRepository,
        session_token_repository: &dyn SessionTokenRepository,
        user_auth: &UserLoginCredentials,
    ) -> Result<SessionTokenModel, UserAuthServiceError> {
        tracing::trace!(method = "login",);

        let user = user_auth_repository
            .get_by_email(db_context, user_auth.email.as_str())
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

impl From<RepositoryError> for UserAuthServiceError {
    fn from(err: RepositoryError) -> Self {
        tracing::error!(error = %err);

        match err {
            RepositoryError::QueryFailed(query_err) => match query_err {
                QueryFailure::AlreadyExists => Self::AlreadyExists,
                QueryFailure::NotCreated => Self::NotCreated,
                QueryFailure::NotFound => Self::Credentials,

                _ => Self::InternalError,
            },

            RepositoryError::InternalError => Self::InternalError,
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
