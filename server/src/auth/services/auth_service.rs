use std::sync::Arc;

use bcrypt::{hash, verify, DEFAULT_COST};

use crate::{
    auth::models::{AuthModel, RegisterModel, SessionTokenModel},
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
            .map_err(|_| AuthServiceError::Credentials)?;

        Self::verify_password(user_auth.password.as_str(), user.password_hash.as_str())?;

        let session_token = SessionTokenService::create_session_token(
            db_context,
            session_token_repository,
            &user.id,
        )
        .await
        .map_err(|_| AuthServiceError::Token)?;

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
            .map_err(|err| match err {
                UserServiceError::AlreadyExists => AuthServiceError::AlreadyExists,
                _ => AuthServiceError::NotCreated,
            })
    }

    fn hash_password(password: &str) -> Result<String, AuthServiceError> {
        hash(password, DEFAULT_COST).map_err(|_| AuthServiceError::NotCreated)
    }

    fn verify_password(password: &str, hash: &str) -> Result<(), AuthServiceError> {
        let valid_password = verify(password, hash).map_err(|_| AuthServiceError::Credentials)?;

        if !valid_password {
            return Err(AuthServiceError::Credentials);
        }

        Ok(())
    }
}

pub enum AuthServiceError {
    Token,
    Credentials,
    NotCreated,
    AlreadyExists,
}
