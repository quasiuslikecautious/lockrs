use bcrypt::verify;
use diesel_async::AsyncPgConnection;

use crate::{
    auth::models::{AuthModel, SessionTokenModel},
    redis::AsyncRedisConnection,
    services::UserService,
};

use super::SessionTokenService;

pub struct AuthService;

impl AuthService {
    pub async fn login(
        pg_connection: &mut AsyncPgConnection,
        redis_connection: &mut AsyncRedisConnection,
        user_auth: &AuthModel,
    ) -> Result<SessionTokenModel, AuthServiceError> {
        let user = UserService::get_user_by_email(pg_connection, &user_auth.email)
            .await
            .map_err(|_| AuthServiceError::Credentials)?;

        let valid_password = verify(&user_auth.password, &user.password_hash)
            .map_err(|_| AuthServiceError::Credentials)?;

        if !valid_password {
            return Err(AuthServiceError::Credentials);
        }

        let session_token = SessionTokenService::create_session_token(redis_connection, &user.id)
            .await
            .map_err(|_| AuthServiceError::Token)?;

        Ok(session_token)
    }
}

pub enum AuthServiceError {
    Token,
    Credentials,
}
