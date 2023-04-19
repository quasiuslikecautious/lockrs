use bcrypt::verify;
use chrono::{Duration, Utc};
use diesel_async::AsyncPgConnection;

use crate::{
    auth::models::{AuthModel, Claims, SessionModel},
    services::{UserService, UserServiceError},
    utils::jwt::JwtUtil,
};

pub struct AuthService;

impl AuthService {
    pub async fn login(
        connection: &mut AsyncPgConnection,
        jwt_util: &JwtUtil,
        user_auth: &AuthModel,
    ) -> Result<SessionModel, AuthServiceError> {
        let user = match UserService::get_user_by_email(connection, &user_auth.email).await {
            Ok(user) => user,
            Err(err) => match err {
                UserServiceError::NotFoundError => return Err(AuthServiceError::InvalidEmail),
                _ => return Err(AuthServiceError::DbError),
            },
        };

        let valid_password = verify(&user_auth.password, &user.password_hash)
            .map_err(|_| AuthServiceError::HashError)?;

        if !valid_password {
            return Err(AuthServiceError::InvalidPassword);
        }

        let now = Utc::now();

        // TODO more dynamic field assignment (aud, iss, etc.)
        let claims = Claims {
            sub: user.id,
            iss: String::from("lockrs"),
            aud: Some(String::from("127.0.0.1:8080")),
            iat: now.timestamp(),
            nbf: now.timestamp(),
            exp: (now + Duration::minutes(5)).timestamp(),
        };

        let token = jwt_util
            .sign_jwt::<Claims>(claims)
            .map_err(|_| AuthServiceError::TokenError)?;

        // TODO add session id generation
        Ok(SessionModel {
            id: user.id.to_string(),
            token,
        })
    }
}

pub enum AuthServiceError {
    DbError,
    HashError,
    TokenError,
    InvalidEmail,
    InvalidPassword,
    NotFound,
}

impl From<diesel::result::Error> for AuthServiceError {
    fn from(diesel_error: diesel::result::Error) -> Self {
        match diesel_error {
            diesel::result::Error::NotFound => Self::NotFound,
            _ => Self::DbError,
        }
    }
}
