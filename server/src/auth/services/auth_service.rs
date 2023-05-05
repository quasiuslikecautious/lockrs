use base64::{engine::general_purpose, Engine as _};
use bcrypt::verify;
use chrono::{Duration, Utc};
use deadpool_redis::redis::cmd;
use diesel_async::AsyncPgConnection;
use rand::Rng;

use crate::{
    auth::models::{AuthModel, Claims, SessionTokenModel},
    redis::AsyncRedisConnection,
    services::{UserService, UserServiceError},
};

pub struct AuthService;

impl AuthService {
    pub async fn login(
        pg_connection: &mut AsyncPgConnection,
        redis_connection: &mut AsyncRedisConnection,
        user_auth: &AuthModel,
    ) -> Result<SessionTokenModel, AuthServiceError> {
        let user = match UserService::get_user_by_email(pg_connection, &user_auth.email).await {
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

        let token = Self::generate_session_token();

        cmd("SET")
            .arg(format!("session_token:{}", token).as_str())
            .arg(serde_json::to_string(&claims).unwrap().as_str())
            .query_async::<_, ()>(redis_connection)
            .await
            .unwrap();

        Ok(SessionTokenModel { token })
    }

    fn generate_session_token() -> String {
        let mut rng = rand::thread_rng();
        let bytes = (0..32).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();

        general_purpose::URL_SAFE_NO_PAD.encode(bytes)
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
